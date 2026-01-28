use crate::error::Result;
use crate::font::Font;
use crate::stream::calculate_checksum;

/// Validation report for a font
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub message: String,
    pub table: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub warning_type: ValidationWarningType,
    pub message: String,
    pub table: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorType {
    InvalidSignature,
    InvalidChecksum,
    MissingRequiredTable,
    InvalidTableStructure,
    InvalidGlyphData,
    InvalidCmapData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationWarningType {
    NonStandardTable,
    UnexpectedTableVersion,
    DeprecatedTable,
    PotentiallyProblematic,
}

impl Font {
    /// Validate the font structure and return a validation report
    pub fn validate(&self) -> Result<ValidationReport> {
        let mut report = ValidationReport {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Validate SFNT signature
        if self.sfnt_version != 0x00010000 && self.sfnt_version != 0x4F54544F {
            report.errors.push(ValidationError {
                error_type: ValidationErrorType::InvalidSignature,
                message: format!("Invalid SFNT version: {:#x}", self.sfnt_version),
                table: None,
            });
            report.is_valid = false;
        }

        // Check for required tables
        let required_tables = [
            (b"cmap", "character to glyph mapping"),
            (b"head", "font header"),
            (b"hhea", "horizontal header"),
            (b"hmtx", "horizontal metrics"),
            (b"maxp", "maximum profile"),
            (b"name", "naming table"),
            (b"OS/2", "OS/2 and Windows metrics"),
            (b"post", "PostScript information"),
        ];

        for (tag, description) in &required_tables {
            if self.get_table_record(tag).is_none() {
                report.errors.push(ValidationError {
                    error_type: ValidationErrorType::MissingRequiredTable,
                    message: format!("Missing required table: {} ({})",
                        String::from_utf8_lossy(tag.as_slice()), description),
                    table: Some(String::from_utf8_lossy(tag.as_slice()).to_string()),
                });
                report.is_valid = false;
            }
        }

        // TrueType fonts require glyf and loca tables
        if self.sfnt_version == 0x00010000 {
            let ttf_required = [(b"glyf", "glyph data"), (b"loca", "index to location")];
            for (tag, description) in &ttf_required {
                if self.get_table_record(tag).is_none() {
                    report.errors.push(ValidationError {
                        error_type: ValidationErrorType::MissingRequiredTable,
                        message: format!("Missing required table for TrueType: {} ({})",
                            String::from_utf8_lossy(tag.as_slice()), description),
                        table: Some(String::from_utf8_lossy(tag.as_slice()).to_string()),
                    });
                    report.is_valid = false;
                }
            }
        }

        // Validate table checksums
        for record in &self.table_records {
            if let Some(table_data) = self.get_table_data(&record.table_tag) {
                let calculated_checksum = calculate_checksum(&table_data);

                // The head table checksum adjustment should be 0xB1B0AFBA
                if record.table_tag == *b"head" {
                    // For head table, we need to skip the checksum adjustment field when calculating
                    // The checksum adjustment is at offset 8, and should be 0xB1B0AFBA - calculated_checksum
                } else if calculated_checksum != record.checksum {
                    report.warnings.push(ValidationWarning {
                        warning_type: ValidationWarningType::PotentiallyProblematic,
                        message: format!(
                            "Checksum mismatch for table {}: expected {:#x}, got {:#x}",
                            String::from_utf8_lossy(&record.table_tag),
                            record.checksum,
                            calculated_checksum
                        ),
                        table: Some(String::from_utf8_lossy(&record.table_tag).to_string()),
                    });
                }
            }
        }

        // Validate head table magic number
        if let Ok(head) = self.head_table() {
            if head.magic_number != 0x5F0F3CF5 {
                report.errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidTableStructure,
                    message: format!("Invalid magic number in head table: {:#x}", head.magic_number),
                    table: Some("head".to_string()),
                });
                report.is_valid = false;
            }

            // Check units per em is valid
            if head.units_per_em == 0 || head.units_per_em > 16384 {
                report.warnings.push(ValidationWarning {
                    warning_type: ValidationWarningType::PotentiallyProblematic,
                    message: format!("Unusual units_per_em value: {}", head.units_per_em),
                    table: Some("head".to_string()),
                });
            }
        }

        // Validate maxp table
        if let Ok(maxp) = self.maxp_table() {
            if maxp.num_glyphs == 0 {
                report.errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidGlyphData,
                    message: "Font has no glyphs".to_string(),
                    table: Some("maxp".to_string()),
                });
                report.is_valid = false;
            }
        }

        // Validate cmap table
        if let Ok(cmap) = self.cmap_table() {
            if cmap.subtables.is_empty() {
                report.errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidCmapData,
                    message: "Cmap table has no subtables".to_string(),
                    table: Some("cmap".to_string()),
                });
                report.is_valid = false;
            }
        }

        // Check for non-standard tables
        let standard_tables = [
            "cmap", "head", "hhea", "hmtx", "maxp", "name", "OS/2", "post",
            "glyf", "loca", "kern", "GPOS", "GSUB", "BASE", "GDEF", "JSTF",
            "vhea", "vmtx", "VORG", "CVT ", "fpgm", "prep", "gasp", "EBSC",
            "trak", "ltsh", "PCLT", "VDMX", "hdmx", "CBDT", "CBLC", "COLR",
            "CPAL", "sbix", "acnt", "avar", "bdat", "bloc", "bsln", "cvar",
            "fdsc", "feat", "fmtx", "fvar", "gvar", "gcid", "glyf", "hvar",
            "just", "lcar", "mort", "morx", "opbd", "prop", "trak", "Zapf",
            "Silf", "Glat", "Gloc", "Feat", "Sill",
        ];

        for record in &self.table_records {
            let tag_str = String::from_utf8_lossy(&record.table_tag).to_string();
            if !standard_tables.contains(&tag_str.as_str()) {
                report.warnings.push(ValidationWarning {
                    warning_type: ValidationWarningType::NonStandardTable,
                    message: format!("Non-standard table: {}", tag_str),
                    table: Some(tag_str),
                });
            }
        }

        Ok(report)
    }

    /// Quick check if the font is valid (returns only boolean)
    pub fn is_valid(&self) -> Result<bool> {
        Ok(self.validate()?.is_valid)
    }
}

impl ValidationReport {
    /// Get a human-readable summary of the validation report
    pub fn summary(&self) -> String {
        let mut summary = String::new();

        if self.is_valid {
            summary.push_str("✓ Font is valid\n");
        } else {
            summary.push_str("✗ Font is invalid\n");
        }

        if !self.errors.is_empty() {
            summary.push_str(&format!("\nErrors ({}):\n", self.errors.len()));
            for error in &self.errors {
                let table = error.table.as_ref().map(|t| format!("[{}] ", t)).unwrap_or_default();
                summary.push_str(&format!("  ✗ {}: {}\n", table, error.message));
            }
        }

        if !self.warnings.is_empty() {
            summary.push_str(&format!("\nWarnings ({}):\n", self.warnings.len()));
            for warning in &self.warnings {
                let table = warning.table.as_ref().map(|t| format!("[{}] ", t)).unwrap_or_default();
                summary.push_str(&format!("  ⚠ {}: {}\n", table, warning.message));
            }
        }

        summary
    }
}
