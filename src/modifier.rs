use crate::error::Result;
use crate::font::Font;
use crate::stream::FontWriter;
use crate::tables::name::NameTable;
use crate::tables::head::HeadTable;
use crate::tables::hhea::HheaTable;
use crate::tables::os2::Os2Table;
use crate::tables::hmtx::HmtxTable;
use crate::tables::TtfTableWrite;
use std::collections::HashMap;

/// FontModifier allows for modifying font properties before saving
pub struct FontModifier {
    font: Font,
    modified_tables: HashMap<[u8; 4], Vec<u8>>,
}

impl FontModifier {
    pub fn new(font: Font) -> Self {
        Self {
            font,
            modified_tables: HashMap::new(),
        }
    }

    /// Set the font family name (name ID 1)
    pub fn set_font_name(&mut self, name: &str) -> Result<&mut Self> {
        let mut name_table = self.font.name_table()?;

        let platform_id = 3u16;
        let encoding_id = 1u16;
        let language_id = 0x0409u16;
        let name_id = 1u16;

        name_table.set_name(name, platform_id, encoding_id, language_id, name_id);
        self.serialize_name_table(name_table)?;
        Ok(self)
    }

    /// Set the full font name (name ID 4)
    pub fn set_full_font_name(&mut self, name: &str) -> Result<&mut Self> {
        let mut name_table = self.font.name_table()?;

        let platform_id = 3u16;
        let encoding_id = 1u16;
        let language_id = 0x0409u16;
        let name_id = 4u16;

        name_table.set_name(name, platform_id, encoding_id, language_id, name_id);
        self.serialize_name_table(name_table)?;
        Ok(self)
    }

    /// Set version string (name ID 5)
    pub fn set_version(&mut self, major: u16, minor: u16) -> Result<&mut Self> {
        let version_string = format!("Version {}.{}", major, minor);

        let mut name_table = self.font.name_table()?;

        let platform_id = 3u16;
        let encoding_id = 1u16;
        let language_id = 0x0409u16;
        let name_id = 5u16;

        name_table.set_name(&version_string, platform_id, encoding_id, language_id, name_id);
        self.serialize_name_table(name_table)?;

        // Also update the font revision in head table
        let mut head_table = self.font.head_table()?;
        head_table.font_revision = (major as f32) + (minor as f32) / 100.0;
        self.serialize_head_table(head_table)?;

        Ok(self)
    }

    /// Set copyright notice (name ID 0)
    pub fn set_copyright(&mut self, copyright: &str) -> Result<&mut Self> {
        let mut name_table = self.font.name_table()?;

        let platform_id = 3u16;
        let encoding_id = 1u16;
        let language_id = 0x0409u16;
        let name_id = 0u16;

        name_table.set_name(copyright, platform_id, encoding_id, language_id, name_id);
        self.serialize_name_table(name_table)?;
        Ok(self)
    }

    /// Set trademark (name ID 7)
    pub fn set_trademark(&mut self, trademark: &str) -> Result<&mut Self> {
        let mut name_table = self.font.name_table()?;

        let platform_id = 3u16;
        let encoding_id = 1u16;
        let language_id = 0x0409u16;
        let name_id = 7u16;

        name_table.set_name(trademark, platform_id, encoding_id, language_id, name_id);
        self.serialize_name_table(name_table)?;
        Ok(self)
    }

    /// Update font version and revision in head table
    pub fn set_font_revision(&mut self, major: u16, minor: u16) -> Result<&mut Self> {
        let mut head_table = self.font.head_table()?;
        head_table.font_revision = (major as f32) + (minor as f32) / 100.0;
        self.serialize_head_table(head_table)?;
        Ok(self)
    }

    /// Modify embedding permissions in OS/2 table
    pub fn set_embedding_type(&mut self, embedding_type: u16) -> Result<&mut Self> {
        let mut os2_table = self.font.os2_table()?;
        os2_table.fs_type = embedding_type;
        self.serialize_os2_table(os2_table)?;
        Ok(self)
    }

    /// Set font name in multiple languages
    pub fn set_localized_font_name(&mut self, name: &str, language_id: u16) -> Result<&mut Self> {
        let mut name_table = self.font.name_table()?;

        let platform_id = 3u16;
        let encoding_id = 1u16;
        let name_id = 1u16;

        name_table.set_name(name, platform_id, encoding_id, language_id, name_id);
        self.serialize_name_table(name_table)?;
        Ok(self)
    }

    /// Update font metrics in head and hhea tables
    pub fn set_font_metrics(&mut self, units_per_em: u16, ascender: i16, descender: i16, line_gap: i16) -> Result<&mut Self> {
        // Update head table
        let mut head_table = self.font.head_table()?;
        head_table.units_per_em = units_per_em;
        self.serialize_head_table(head_table)?;

        // Update hhea table (field names are ascent/descent, not ascender/descender)
        let mut hhea_table = self.font.hhea_table()?;
        hhea_table.ascent = ascender;
        hhea_table.descent = descender;
        hhea_table.line_gap = line_gap;
        self.serialize_hhea_table(hhea_table)?;

        Ok(self)
    }

    /// Modify glyph advance widths
    pub fn set_glyph_advance(&mut self, glyph_index: usize, advance_width: u16) -> Result<&mut Self> {
        let mut hmtx_table = self.font.hmtx_table()?;
        let hhea_table = self.font.hhea_table()?;

        if glyph_index < hmtx_table.h_metrics.len() {
            if glyph_index < hhea_table.number_of_h_metrics as usize {
                hmtx_table.h_metrics[glyph_index].advance_width = advance_width;
            }
            self.serialize_hmtx_table(hmtx_table)?;
        }

        Ok(self)
    }

    /// Serialize modified name table
    fn serialize_name_table(&mut self, table: NameTable) -> Result<()> {
        let mut writer = FontWriter::new();
        table.write(&mut writer)?;
        self.modified_tables.insert(*b"name", writer.into_inner());
        Ok(())
    }

    /// Serialize modified head table
    fn serialize_head_table(&mut self, table: HeadTable) -> Result<()> {
        let mut writer = FontWriter::new();
        table.write(&mut writer)?;
        self.modified_tables.insert(*b"head", writer.into_inner());
        Ok(())
    }

    /// Serialize modified hhea table
    fn serialize_hhea_table(&mut self, table: HheaTable) -> Result<()> {
        let mut writer = FontWriter::new();
        table.write(&mut writer)?;
        self.modified_tables.insert(*b"hhea", writer.into_inner());
        Ok(())
    }

    /// Serialize modified OS/2 table
    fn serialize_os2_table(&mut self, table: Os2Table) -> Result<()> {
        let mut writer = FontWriter::new();
        table.write(&mut writer)?;
        self.modified_tables.insert(*b"OS/2", writer.into_inner());
        Ok(())
    }

    /// Serialize modified hmtx table
    fn serialize_hmtx_table(&mut self, table: HmtxTable) -> Result<()> {
        let mut writer = FontWriter::new();
        table.write(&mut writer)?;
        self.modified_tables.insert(*b"hmtx", writer.into_inner());
        Ok(())
    }

    /// Commit all modifications and return the modified font
    pub fn commit(mut self) -> Result<Font> {
        // Apply all modified tables to the font
        for (tag, data) in &self.modified_tables {
            // Find the table record and update it
            if let Some(record) = self.font.table_records.iter_mut().find(|r| r.table_tag == *tag) {
                record.length = data.len() as u32;
                // Offset will be recalculated during save
            }
        }

        // Create new data with modified tables
        let mut new_data = self.font.data.clone();

        // Update the data with modified tables (simplified approach)
        for (tag, data) in &self.modified_tables {
            if let Some(record) = self.font.get_table_record(tag) {
                let offset = record.offset as usize;
                // Make sure we have enough space
                if offset + data.len() <= new_data.len() {
                    new_data[offset..offset + data.len()].copy_from_slice(data);
                }
            }
        }

        self.font.data = new_data;
        Ok(self.font)
    }
}

impl Font {
    pub fn modify(self) -> FontModifier {
        FontModifier::new(self)
    }
}
