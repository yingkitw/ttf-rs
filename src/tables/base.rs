use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// BASE table - Baseline data
#[derive(Debug, Clone)]
pub struct BaseTable {
    pub version: u32,
    pub horiz_axis_offset: u32,
    pub vert_axis_offset: u32,
}

impl TtfTable for BaseTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let horiz_axis_offset = reader.read_u32()?;
        let vert_axis_offset = reader.read_u32()?;

        Ok(BaseTable {
            version,
            horiz_axis_offset,
            vert_axis_offset,
        })
    }
}
