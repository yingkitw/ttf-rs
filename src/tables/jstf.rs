use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// JSTF table - Justification data
#[derive(Debug, Clone)]
pub struct JstfTable {
    pub version: u32,
    pub gsub_lookup_count: u16,
    pub gsub_lookup_offsets: Vec<u32>,
    pub gpos_lookup_count: u16,
    pub gpos_lookup_offsets: Vec<u32>,
}

impl TtfTable for JstfTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let gsub_lookup_count = reader.read_u16()?;
        let _gsub_offset = reader.read_u16()?;

        let mut gsub_lookup_offsets = Vec::with_capacity(gsub_lookup_count as usize);
        for _ in 0..gsub_lookup_count {
            gsub_lookup_offsets.push(reader.read_u32()?);
        }

        let gpos_lookup_count = reader.read_u16()?;
        let _gpos_offset = reader.read_u16()?;

        let mut gpos_lookup_offsets = Vec::with_capacity(gpos_lookup_count as usize);
        for _ in 0..gpos_lookup_count {
            gpos_lookup_offsets.push(reader.read_u32()?);
        }

        Ok(JstfTable {
            version,
            gsub_lookup_count,
            gsub_lookup_offsets,
            gpos_lookup_count,
            gpos_lookup_offsets,
        })
    }
}
