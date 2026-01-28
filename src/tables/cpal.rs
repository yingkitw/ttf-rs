use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// CPAL table - Color palette table
#[derive(Debug, Clone)]
pub struct CpalTable {
    pub version: u16,
    pub num_palette_entries: u16,
    pub num_palettes: u16,
    pub num_color_records: u32,
    pub palettes: Vec<Palette>,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub colors: Vec<ColorRecord>,
}

#[derive(Debug, Clone)]
pub struct ColorRecord {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

impl TtfTable for CpalTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let num_palette_entries = reader.read_u16()?;
        let num_palettes = reader.read_u16()?;
        let num_color_records = reader.read_u32()?;

        let mut palettes = Vec::new();

        for _ in 0..num_palettes {
            let mut colors = Vec::new();
            for _ in 0..num_palette_entries {
                let blue = reader.read_u8()?;
                let green = reader.read_u8()?;
                let red = reader.read_u8()?;
                let alpha = reader.read_u8()?;
                colors.push(ColorRecord {
                    blue,
                    green,
                    red,
                    alpha,
                });
            }
            palettes.push(Palette { colors });
        }

        Ok(CpalTable {
            version,
            num_palette_entries,
            num_palettes,
            num_color_records,
            palettes,
        })
    }
}
