use crate::error::Result;
use crate::stream::FontReader;

/// LOCA table - Index to location
#[derive(Debug, Clone)]
pub enum LocaTable {
    Short(Vec<u16>),
    Long(Vec<u32>),
}

impl LocaTable {
    pub fn from_reader(reader: &mut FontReader, _length: u32, num_glyphs: usize, is_long: bool) -> Result<Self> {
        if is_long {
            let mut offsets = Vec::with_capacity(num_glyphs + 1);
            for _ in 0..=num_glyphs {
                offsets.push(reader.read_u32()?);
            }
            Ok(LocaTable::Long(offsets))
        } else {
            let mut offsets = Vec::with_capacity(num_glyphs + 1);
            for _ in 0..=num_glyphs {
                offsets.push(reader.read_u16()?);
            }
            Ok(LocaTable::Short(offsets))
        }
    }

    pub fn get_offset(&self, index: usize) -> Result<u32> {
        match self {
            LocaTable::Short(offsets) => {
                let offset = offsets.get(index).copied().ok_or_else(|| {
                    crate::error::TtfError::InvalidGlyphIndex(index as u16)
                })?;
                Ok(offset as u32 * 2)
            }
            LocaTable::Long(offsets) => {
                offsets.get(index).copied().ok_or_else(|| {
                    crate::error::TtfError::InvalidGlyphIndex(index as u16)
                })
            }
        }
    }
}
