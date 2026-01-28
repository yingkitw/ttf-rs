use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// PREP table - Control value program
#[derive(Debug, Clone)]
pub struct PrepTable {
    pub instructions: Vec<u8>,
}

impl TtfTable for PrepTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let instructions = reader.read_bytes(length as usize)?;
        Ok(PrepTable { instructions })
    }
}
