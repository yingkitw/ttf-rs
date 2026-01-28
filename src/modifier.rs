use crate::error::Result;
use crate::font::Font;
use crate::tables::name::NameRecord;
use std::collections::HashMap;

pub struct FontModifier {
    font: Font,
    modified_tables: HashMap<String, Vec<u8>>,
}

impl FontModifier {
    pub fn new(font: Font) -> Self {
        Self {
            font,
            modified_tables: HashMap::new(),
        }
    }

    pub fn set_font_name(&mut self, name: &str) -> Result<()> {
        let mut name_table = self.font.name_table()?;
        
        let platform_id = 3;
        let encoding_id = 1;
        let language_id = 0x0409;
        let name_id = 1;
        
        let name_bytes: Vec<u16> = name.encode_utf16().collect();
        let mut name_data = Vec::new();
        for code_unit in name_bytes {
            name_data.extend_from_slice(&code_unit.to_be_bytes());
        }
        
        let record = NameRecord {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length: name_data.len() as u16,
            offset: 0,
        };
        
        name_table.name_records.retain(|r| {
            !(r.platform_id == platform_id
                && r.encoding_id == encoding_id
                && r.language_id == language_id
                && r.name_id == name_id)
        });
        name_table.name_records.push(record);
        
        Ok(())
    }

    pub fn set_full_font_name(&mut self, name: &str) -> Result<()> {
        let mut name_table = self.font.name_table()?;
        
        let platform_id = 3;
        let encoding_id = 1;
        let language_id = 0x0409;
        let name_id = 4;
        
        let name_bytes: Vec<u16> = name.encode_utf16().collect();
        let mut name_data = Vec::new();
        for code_unit in name_bytes {
            name_data.extend_from_slice(&code_unit.to_be_bytes());
        }
        
        let record = NameRecord {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length: name_data.len() as u16,
            offset: 0,
        };
        
        name_table.name_records.retain(|r| {
            !(r.platform_id == platform_id
                && r.encoding_id == encoding_id
                && r.language_id == language_id
                && r.name_id == name_id)
        });
        name_table.name_records.push(record);
        
        Ok(())
    }

    pub fn set_version(&mut self, major: u16, minor: u16) -> Result<()> {
        let version_string = format!("Version {}.{}", major, minor);
        
        let mut name_table = self.font.name_table()?;
        
        let platform_id = 3;
        let encoding_id = 1;
        let language_id = 0x0409;
        let name_id = 5;
        
        let name_bytes: Vec<u16> = version_string.encode_utf16().collect();
        let mut name_data = Vec::new();
        for code_unit in name_bytes {
            name_data.extend_from_slice(&code_unit.to_be_bytes());
        }
        
        let record = NameRecord {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length: name_data.len() as u16,
            offset: 0,
        };
        
        name_table.name_records.retain(|r| {
            !(r.platform_id == platform_id
                && r.encoding_id == encoding_id
                && r.language_id == language_id
                && r.name_id == name_id)
        });
        name_table.name_records.push(record);
        
        Ok(())
    }

    pub fn set_copyright(&mut self, copyright: &str) -> Result<()> {
        let mut name_table = self.font.name_table()?;
        
        let platform_id = 3;
        let encoding_id = 1;
        let language_id = 0x0409;
        let name_id = 0;
        
        let name_bytes: Vec<u16> = copyright.encode_utf16().collect();
        let mut name_data = Vec::new();
        for code_unit in name_bytes {
            name_data.extend_from_slice(&code_unit.to_be_bytes());
        }
        
        let record = NameRecord {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length: name_data.len() as u16,
            offset: 0,
        };
        
        name_table.name_records.retain(|r| {
            !(r.platform_id == platform_id
                && r.encoding_id == encoding_id
                && r.language_id == language_id
                && r.name_id == name_id)
        });
        name_table.name_records.push(record);
        
        Ok(())
    }

    pub fn set_trademark(&mut self, trademark: &str) -> Result<()> {
        let mut name_table = self.font.name_table()?;
        
        let platform_id = 3;
        let encoding_id = 1;
        let language_id = 0x0409;
        let name_id = 7;
        
        let name_bytes: Vec<u16> = trademark.encode_utf16().collect();
        let mut name_data = Vec::new();
        for code_unit in name_bytes {
            name_data.extend_from_slice(&code_unit.to_be_bytes());
        }
        
        let record = NameRecord {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length: name_data.len() as u16,
            offset: 0,
        };
        
        name_table.name_records.retain(|r| {
            !(r.platform_id == platform_id
                && r.encoding_id == encoding_id
                && r.language_id == language_id
                && r.name_id == name_id)
        });
        name_table.name_records.push(record);
        
        Ok(())
    }

    pub fn commit(self) -> Result<Font> {
        Ok(self.font)
    }
}

impl Font {
    pub fn modify(self) -> FontModifier {
        FontModifier::new(self)
    }
}
