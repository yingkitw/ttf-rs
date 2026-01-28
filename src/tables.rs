// Table module for TTF table parsing

pub mod head;
pub mod maxp;
pub mod cmap;
pub mod name;
pub mod hhea;
pub mod hmtx;
pub mod glyf;
pub mod loca;
pub mod post;
pub mod os2;

use crate::error::Result;
use crate::stream::{FontReader, FontWriter};

/// Table record in the SFNT header
#[derive(Debug, Clone)]
pub struct TableRecord {
    pub table_tag: [u8; 4],
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

impl TableRecord {
    pub fn tag_to_string(&self) -> String {
        String::from_utf8_lossy(&self.table_tag).to_string()
    }

    pub fn from_reader(reader: &mut FontReader) -> Result<Self> {
        let table_tag = reader.read_tag()?;
        let checksum = reader.read_u32()?;
        let offset = reader.read_u32()?;
        let length = reader.read_u32()?;

        Ok(TableRecord {
            table_tag,
            checksum,
            offset,
            length,
        })
    }

    pub fn write(&self, writer: &mut FontWriter) {
        writer.write_tag(&self.table_tag);
        writer.write_u32(self.checksum);
        writer.write_u32(self.offset);
        writer.write_u32(self.length);
    }

    pub fn new(tag: [u8; 4], offset: u32, length: u32) -> Self {
        TableRecord {
            table_tag: tag,
            checksum: 0,
            offset,
            length,
        }
    }
}

/// Trait that all TTF tables must implement for reading
pub trait TtfTable: Sized {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self>;

    fn table_tag() -> Option<&'static [u8; 4]> {
        None
    }
}

/// Trait for tables that can be written
pub trait TtfTableWrite: Sized {
    fn write(&self, writer: &mut FontWriter) -> Result<()>;

    fn table_tag() -> &'static [u8; 4];
}

