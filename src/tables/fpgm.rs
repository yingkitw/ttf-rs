use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// FPGM table - Font program
#[derive(Debug, Clone)]
pub struct FpgmTable {
    pub instructions: Vec<u8>,
}

impl TtfTable for FpgmTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let instructions = reader.read_bytes(length as usize)?;
        Ok(FpgmTable { instructions })
    }
}
