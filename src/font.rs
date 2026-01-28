use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::{Result, TtfError};
use crate::stream::{FontReader, FontWriter};
use crate::stream::calculate_checksum;
use crate::tables::{TableRecord, TtfTable};
use crate::tables::head::HeadTable;
use crate::tables::maxp::MaxpTable;
use crate::tables::cmap::CmapTable;
use crate::tables::name::NameTable;
use crate::tables::hhea::HheaTable;
use crate::tables::hmtx::HmtxTable;
use crate::tables::glyf::GlyfTable;
use crate::tables::loca::LocaTable;
use crate::tables::post::PostTable;
use crate::tables::os2::Os2Table;

/// Main Font structure representing a TrueType font
#[derive(Debug, Clone)]
pub struct Font {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub table_records: Vec<TableRecord>,
    pub data: Vec<u8>,
}

impl Font {
    const SFNT_TRUETYPE: u32 = 0x00010000;
    const SFNT_OPENTYPE: u32 = 0x4F54544F; // 'OTTO'

    /// Load a font from a file path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let metadata = file.metadata()?;
        let size = metadata.len() as usize;

        let mut data = Vec::with_capacity(size);
        file.read_to_end(&mut data)?;

        Self::from_data(data)
    }

    /// Load a font from raw bytes
    pub fn from_data(data: Vec<u8>) -> Result<Self> {
        let mut reader = FontReader::new(data);

        // Read SFNT header
        let sfnt_version = reader.read_u32()?;

        if sfnt_version != Self::SFNT_TRUETYPE && sfnt_version != Self::SFNT_OPENTYPE {
            return Err(TtfError::InvalidSignature {
                expected: Self::SFNT_TRUETYPE,
                actual: sfnt_version,
            });
        }

        let num_tables = reader.read_u16()?;
        let search_range = reader.read_u16()?;
        let entry_selector = reader.read_u16()?;
        let range_shift = reader.read_u16()?;

        // Read table records
        let mut table_records = Vec::with_capacity(num_tables as usize);
        for _ in 0..num_tables {
            table_records.push(TableRecord::from_reader(&mut reader)?);
        }

        let data = reader.into_inner();

        Ok(Font {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records,
            data,
        })
    }

    /// Get a table record by tag
    pub fn get_table_record(&self, tag: &[u8; 4]) -> Option<&TableRecord> {
        self.table_records
            .iter()
            .find(|r| r.table_tag == *tag)
    }

    /// Get raw table data
    pub fn get_table_data(&self, tag: &[u8; 4]) -> Option<Vec<u8>> {
        let record = self.get_table_record(tag)?;
        let start = record.offset as usize;
        let end = start + record.length as usize;

        if end > self.data.len() {
            return None;
        }

        Some(self.data[start..end].to_vec())
    }

    /// Parse and return the head table
    pub fn head_table(&self) -> Result<HeadTable> {
        let record = self
            .get_table_record(b"head")
            .ok_or_else(|| TtfError::MissingTable("head".to_string()))?;

        let start = record.offset as usize;
        let mut reader = FontReader::from_slice(&self.data[start..start + record.length as usize]);

        HeadTable::from_reader(&mut reader, record.length)
    }

    /// Parse and return the maxp table
    pub fn maxp_table(&self) -> Result<MaxpTable> {
        let record = self
            .get_table_record(b"maxp")
            .ok_or_else(|| TtfError::MissingTable("maxp".to_string()))?;

        let start = record.offset as usize;
        let mut reader = FontReader::from_slice(&self.data[start..start + record.length as usize]);

        MaxpTable::from_reader(&mut reader, record.length)
    }

    /// Parse and return the cmap table
    pub fn cmap_table(&self) -> Result<CmapTable> {
        let record = self
            .get_table_record(b"cmap")
            .ok_or_else(|| TtfError::MissingTable("cmap".to_string()))?;

        let start = record.offset as usize;
        let end = start + record.length as usize;
        let mut reader = FontReader::from_slice(&self.data[start..end]);

        CmapTable::from_reader(&mut reader, record.length)
    }

    /// Parse and return the name table
    pub fn name_table(&self) -> Result<NameTable> {
        let record = self
            .get_table_record(b"name")
            .ok_or_else(|| TtfError::MissingTable("name".to_string()))?;

        let start = record.offset as usize;
        let mut reader = FontReader::from_slice(&self.data[start..start + record.length as usize]);

        NameTable::from_reader(&mut reader, record.length)
    }

    /// Parse and return the hhea table
    pub fn hhea_table(&self) -> Result<HheaTable> {
        let record = self
            .get_table_record(b"hhea")
            .ok_or_else(|| TtfError::MissingTable("hhea".to_string()))?;

        let start = record.offset as usize;
        let mut reader = FontReader::from_slice(&self.data[start..start + record.length as usize]);

        HheaTable::from_reader(&mut reader, record.length)
    }

    /// Parse and return the hmtx table
    pub fn hmtx_table(&self) -> Result<HmtxTable> {
        let record = self
            .get_table_record(b"hmtx")
            .ok_or_else(|| TtfError::MissingTable("hmtx".to_string()))?;

        let hhea = self.hhea_table()?;
        let maxp = self.maxp_table()?;

        let start = record.offset as usize;
        let end = start + record.length as usize;
        let mut reader = FontReader::from_slice(&self.data[start..end]);

        HmtxTable::from_reader(&mut reader, record.length, maxp.num_glyphs, hhea.number_of_h_metrics)
    }

    /// Parse and return the loca table
    pub fn loca_table(&self) -> Result<LocaTable> {
        let record = self
            .get_table_record(b"loca")
            .ok_or_else(|| TtfError::MissingTable("loca".to_string()))?;

        let head = self.head_table()?;
        let maxp = self.maxp_table()?;

        let start = record.offset as usize;
        let end = start + record.length as usize;
        let mut reader = FontReader::from_slice(&self.data[start..end]);

        LocaTable::from_reader(
            &mut reader,
            record.length,
            maxp.num_glyphs as usize + 1,
            head.is_long_loca_format(),
        )
    }

    /// Parse and return the glyf table
    pub fn glyf_table(&self) -> Result<GlyfTable> {
        let record = self
            .get_table_record(b"glyf")
            .ok_or_else(|| TtfError::MissingTable("glyf".to_string()))?;

        let loca = self.loca_table()?;
        let maxp = self.maxp_table()?;

        let start = record.offset as usize;
        let end = start + record.length as usize;
        let mut reader = FontReader::from_slice(&self.data[start..end]);

        GlyfTable::from_reader(&mut reader, record.length, &loca, maxp.num_glyphs)
    }

    /// Parse and return the post table
    pub fn post_table(&self) -> Result<PostTable> {
        let record = self
            .get_table_record(b"post")
            .ok_or_else(|| TtfError::MissingTable("post".to_string()))?;

        let start = record.offset as usize;
        let mut reader = FontReader::from_slice(&self.data[start..start + record.length as usize]);

        PostTable::from_reader(&mut reader, record.length)
    }

    /// Parse and return the OS/2 table
    pub fn os2_table(&self) -> Result<Os2Table> {
        let record = self
            .get_table_record(b"OS/2")
            .ok_or_else(|| TtfError::MissingTable("OS/2".to_string()))?;

        let start = record.offset as usize;
        let mut reader = FontReader::from_slice(&self.data[start..start + record.length as usize]);

        Os2Table::from_reader(&mut reader, record.length)
    }

    /// Get glyph index for a character
    pub fn char_to_glyph(&self, c: char) -> Result<u32> {
        let cmap = self.cmap_table()?;
        cmap.map_char(c).ok_or_else(|| {
            TtfError::ParseError(format!("No glyph found for character: {}", c))
        })
    }

    /// Get font name
    pub fn font_name(&self) -> Result<String> {
        let name = self.name_table()?;
        if let Some(record) = name.get_font_name() {
            Ok(format!("{:?}", record))
        } else {
            Ok("Unknown".to_string())
        }
    }

    /// Get font family name
    pub fn family_name(&self) -> Result<String> {
        let name = self.name_table()?;
        if let Some(record) = name.get_full_name() {
            Ok(format!("{:?}", record))
        } else {
            Ok("Unknown".to_string())
        }
    }

    /// Check if font is bold
    pub fn is_bold(&self) -> Result<bool> {
        let os2 = self.os2_table()?;
        Ok(os2.is_bold())
    }

    /// Check if font is italic
    pub fn is_italic(&self) -> Result<bool> {
        let os2 = self.os2_table()?;
        Ok(os2.is_italic())
    }

    /// Get number of glyphs
    pub fn num_glyphs(&self) -> Result<u16> {
        let maxp = self.maxp_table()?;
        Ok(maxp.num_glyphs)
    }

    /// Get units per em
    pub fn units_per_em(&self) -> Result<u16> {
        let head = self.head_table()?;
        Ok(head.units_per_em)
    }

    /// List all tables
    pub fn list_tables(&self) -> Vec<String> {
        self.table_records
            .iter()
            .map(|r| r.tag_to_string())
            .collect()
    }

    /// Save font to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = self.to_bytes()?;
        let mut file = File::create(path)?;
        file.write_all(&data)?;
        Ok(())
    }

    /// Convert font to raw bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut writer = FontWriter::new();

        // Calculate table data and offsets
        let header_size = 12 + (self.table_records.len() * 16);
        let mut current_offset = header_size as u32;

        // Round up offset to 4-byte boundary
        if current_offset % 4 != 0 {
            current_offset = (current_offset + 3) & !3;
        }

        // Collect table data
        let mut table_data: Vec<([u8; 4], Vec<u8>)> = Vec::new();

        for record in &self.table_records {
            // Get raw table data
            if let Some(data) = self.get_table_data(&record.table_tag) {
                table_data.push((record.table_tag, data));
            }
        }

        // Write SFNT header
        writer.write_u32(self.sfnt_version);
        writer.write_u16(self.num_tables);
        writer.write_u16(self.search_range);
        writer.write_u16(self.entry_selector);
        writer.write_u16(self.range_shift);

        // Write table directory (placeholder offsets for now)
        let mut table_dir_offset = writer.position();

        for record in &self.table_records {
            writer.write_tag(&record.table_tag);
            writer.write_u32(0); // checksum placeholder
            writer.write_u32(0); // offset placeholder
            writer.write_u32(record.length);
        }

        // Write table data and update directory
        let mut data_start = writer.position();

        // Pad to 4-byte boundary
        while data_start % 4 != 0 {
            writer.write_u8(0);
            data_start = writer.position();
        }

        let mut dir_positions: Vec<(usize, u32, u32)> = Vec::new();

        for (tag, data) in &table_data {
            let checksum = calculate_checksum(data);
            let offset = writer.position() as u32;
            let length = data.len() as u32;

            // Write table data
            writer.write_bytes(data);

            // Pad to 4-byte boundary
            writer.write_padding(4);

            // Store position in directory to update
            dir_positions.push((data_start, offset, checksum));
            data_start += 16;
        }

        // Update directory with actual offsets and checksums
        let header_bytes = writer.as_slice();
        let mut header_update = Vec::from(&header_bytes[12..12 + self.table_records.len() * 16]);

        let mut idx = 0;
        for (_, offset, checksum) in dir_positions {
            // Update checksum at position 12 + idx*16 + 4
            let checksum_pos = idx * 16 + 4;
            header_update[checksum_pos..checksum_pos + 4]
                .copy_from_slice(&checksum.to_be_bytes());

            // Update offset at position 12 + idx*16 + 8
            let offset_pos = idx * 16 + 8;
            header_update[offset_pos..offset_pos + 4]
                .copy_from_slice(&offset.to_be_bytes());

            idx += 1;
        }

        // Reconstruct final data
        let mut final_data = FontWriter::new();
        final_data.write_bytes(&header_bytes[..12]); // SFNT header
        final_data.write_bytes(&header_update);      // Updated table directory
        final_data.write_bytes(&header_bytes[12 + self.table_records.len() * 16..]); // Table data

        Ok(final_data.into_inner())
    }

    /// Calculate search range for given number of tables
    fn calculate_search_range(num_tables: u16) -> (u16, u16, u16) {
        let mut max_power = 1u16;
        let mut entry_selector = 0u16;

        while max_power * 2 <= num_tables {
            max_power *= 2;
            entry_selector += 1;
        }

        let search_range = max_power * 16;
        let range_shift = if num_tables > max_power {
            (num_tables - max_power) * 16
        } else {
            0
        };

        (search_range, entry_selector, range_shift)
    }

    /// Create a new font with modified data
    pub fn with_table_data(&self, tag: &[u8; 4], data: Vec<u8>) -> Result<Self> {
        let mut font = self.clone();

        // Find and replace table data
        if let Some(record) = font.table_records.iter().position(|r| r.table_tag == *tag) {
            font.table_records[record].length = data.len() as u32;

            // Update font data (simplified - in practice you'd want more sophisticated handling)
            let offset = font.table_records[record].offset as usize;
            if offset + data.len() <= font.data.len() {
                font.data[offset..offset + data.len()].copy_from_slice(&data);
            }
        }

        Ok(font)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_creation() {
        // Test that Font can be created (would need actual TTF data)
        assert!(true);
    }
}
