use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// CBDT table - Color bitmap data
#[derive(Debug, Clone)]
pub struct CbdTTable {
    pub version: u32,
    pub num_glyphs: u32,
    pub bitmap_sizes: Vec<BitmapSize>,
}

#[derive(Debug, Clone)]
pub struct BitmapSize {
    pub index: u32,
    pub ppem_x: u16,
    pub ppem_y: u16,
    pub bit_depth: u8,
    pub flags: u8,
}

impl TtfTable for CbdTTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let num_glyphs = reader.read_u32()?;

        let bitmap_sizes = Vec::new();

        Ok(CbdTTable {
            version,
            num_glyphs,
            bitmap_sizes,
        })
    }
}

/// CBLC table - Color bitmap location
#[derive(Debug, Clone)]
pub struct CblcTable {
    pub version: u32,
    pub num_sizes: u32,
    pub bitmap_tables: Vec<u32>,
}

impl TtfTable for CblcTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let num_sizes = reader.read_u32()?;

        let mut bitmap_tables = Vec::new();
        for _ in 0..num_sizes {
            bitmap_tables.push(reader.read_u32()?);
        }

        Ok(CblcTable {
            version,
            num_sizes,
            bitmap_tables,
        })
    }
}
