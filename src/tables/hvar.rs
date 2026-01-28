use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// HVAR table - Horizontal metrics variations
#[derive(Debug, Clone)]
pub struct HvarTable {
    pub version: u32,
    pub variation_region_list_offset: u32,
    pub item_variation_store_offset: u32,
}

/// VVAR table - Vertical metrics variations
#[derive(Debug, Clone)]
pub struct VvarTable {
    pub version: u32,
    pub variation_region_list_offset: u32,
    pub item_variation_store_offset: u32,
}

impl TtfTable for HvarTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let variation_region_list_offset = reader.read_u32()?;
        let item_variation_store_offset = reader.read_u32()?;

        Ok(HvarTable {
            version,
            variation_region_list_offset,
            item_variation_store_offset,
        })
    }
}

impl TtfTable for VvarTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let variation_region_list_offset = reader.read_u32()?;
        let item_variation_store_offset = reader.read_u32()?;

        Ok(VvarTable {
            version,
            variation_region_list_offset,
            item_variation_store_offset,
        })
    }
}
