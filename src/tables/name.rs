use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// NAME table - Naming table
#[derive(Debug, Clone)]
pub struct NameTable {
    pub format: u16,
    pub count: u16,
    pub string_offset: u16,
    pub name_records: Vec<NameRecord>,
}

#[derive(Debug, Clone)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub length: u16,
    pub offset: u16,
}

impl NameRecord {
    pub const COPYRIGHT_NOTICE: u16 = 0;
    pub const FONT_FAMILY_NAME: u16 = 1;
    pub const FONT_SUBFAMILY_NAME: u16 = 2;
    pub const UNIQUE_FONT_ID: u16 = 3;
    pub const FULL_FONT_NAME: u16 = 4;
    pub const VERSION_STRING: u16 = 5;
    pub const POSTSCRIPT_NAME: u16 = 6;
    pub const TRADEMARK: u16 = 7;
    pub const MANUFACTURER_NAME: u16 = 8;
    pub const DESIGNER: u16 = 9;
    pub const DESCRIPTION: u16 = 10;
    pub const VENDOR_URL: u16 = 11;
    pub const DESIGNER_URL: u16 = 12;
    pub const LICENSE_DESCRIPTION: u16 = 13;
    pub const LICENSE_URL: u16 = 14;
    // IDs 15-16 are reserved
    pub const TYPOGRAPHIC_FAMILY_NAME: u16 = 16;
    pub const TYPOGRAPHIC_SUBFAMILY_NAME: u16 = 17;
    pub const COMPATIBLE_FULL_NAME: u16 = 18;
    pub const SAMPLE_TEXT: u16 = 19;
    pub const POSTSCRIPT_CID: u16 = 20;
    pub const WWS_FAMILY_NAME: u16 = 21;
    pub const WWS_SUBFAMILY_NAME: u16 = 22;
    pub const LIGHT_BACKGROUND_PALETTE: u16 = 23;
    pub const DARK_BACKGROUND_PALETTE: u16 = 24;
}

impl NameTable {
    pub fn get_name(&self, name_id: u16) -> Option<(&NameRecord, String)> {
        self.name_records
            .iter()
            .find(|r| r.name_id == name_id)
            .map(|record| (record, String::new())) // String would need to be extracted from string storage
    }

    pub fn get_font_name(&self) -> Option<&NameRecord> {
        self.name_records
            .iter()
            .find(|r| r.name_id == NameRecord::FONT_FAMILY_NAME)
    }

    pub fn get_full_name(&self) -> Option<&NameRecord> {
        self.name_records
            .iter()
            .find(|r| r.name_id == NameRecord::FULL_FONT_NAME)
    }

    pub fn get_postscript_name(&self) -> Option<&NameRecord> {
        self.name_records
            .iter()
            .find(|r| r.name_id == NameRecord::POSTSCRIPT_NAME)
    }
}

impl TtfTable for NameTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let format = reader.read_u16()?;
        let count = reader.read_u16()?;
        let string_offset = reader.read_u16()?;

        let mut name_records = Vec::with_capacity(count as usize);
        for _ in 0..count {
            name_records.push(NameRecord {
                platform_id: reader.read_u16()?,
                encoding_id: reader.read_u16()?,
                language_id: reader.read_u16()?,
                name_id: reader.read_u16()?,
                length: reader.read_u16()?,
                offset: reader.read_u16()?,
            });
        }

        // Note: Actual string data would be read at string_offset
        // For now, we just parse the records

        Ok(NameTable {
            format,
            count,
            string_offset,
            name_records,
        })
    }
}
