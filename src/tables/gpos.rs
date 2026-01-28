use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// GPOS table - Glyph positioning
#[derive(Debug, Clone)]
pub struct GposTable {
    pub version: u32,
    pub script_list_offset: u32,
    pub feature_list_offset: u32,
    pub lookup_list_offset: u32,
    pub feature_variations_offset: Option<u32>,
}

impl TtfTable for GposTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let script_list_offset = reader.read_u32()?;
        let feature_list_offset = reader.read_u32()?;
        let lookup_list_offset = reader.read_u32()?;

        let feature_variations_offset = if version >= 0x00010000 {
            Some(reader.read_u32()?)
        } else {
            None
        };

        Ok(GposTable {
            version,
            script_list_offset,
            feature_list_offset,
            lookup_list_offset,
            feature_variations_offset,
        })
    }
}
