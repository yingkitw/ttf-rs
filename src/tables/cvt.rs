use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// CVT table - Control value table
#[derive(Debug, Clone)]
pub struct CvtTable {
    pub values: Vec<i16>,
}

impl TtfTable for CvtTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let num_values = (length as usize) / 2;
        let mut values = Vec::with_capacity(num_values);
        for _ in 0..num_values {
            values.push(reader.read_i16()?);
        }
        Ok(CvtTable { values })
    }
}
