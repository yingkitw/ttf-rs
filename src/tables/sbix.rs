use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// sbix table - Standard bitmap graphics
#[derive(Debug, Clone)]
pub struct SbixTable {
    pub version: u16,
    pub flags: u16,
    pub strikes: Vec<Strike>,
}

#[derive(Debug, Clone)]
pub struct Strike {
    pub ppem: u16,
    pub resolution: u16,
    pub bitmaps: Vec<Bitmap>,
}

#[derive(Debug, Clone)]
pub struct Bitmap {
    pub glyph_id: u16,
    pub bitmap_type: u32,
    pub offset: u32,
    pub data: Vec<u8>,
}

impl TtfTable for SbixTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let flags = reader.read_u16()?;
        let num_strikes = reader.read_u32()?;

        let mut strikes = Vec::new();

        // Simplified implementation
        for _ in 0..num_strikes {
            let ppem = reader.read_u16()?;
            let resolution = reader.read_u16()?;
            strikes.push(Strike {
                ppem,
                resolution,
                bitmaps: Vec::new(),
            });
        }

        Ok(SbixTable {
            version,
            flags,
            strikes,
        })
    }
}
