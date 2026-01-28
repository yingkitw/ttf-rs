use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// GVAR table - Glyph variations
#[derive(Debug, Clone)]
pub struct GvarTable {
    pub version: u32,
    pub axes_array_offset: u16,
    pub shared_tuples_count: u16,
    pub shared_tuples_offset: u32,
    pub glyph_variation_data: Vec<GlyphVariationData>,
}

#[derive(Debug, Clone)]
pub struct GlyphVariationData {
    pub glyph_id: u16,
    pub variations: Vec<Variation>,
}

#[derive(Debug, Clone)]
pub struct Variation {
    pub tuple_index: u16,
    pub variation_data: Vec<u8>,
}

impl TtfTable for GvarTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let axes_array_offset = reader.read_u16()?;
        let shared_tuples_count = reader.read_u16()?;
        let shared_tuples_offset = reader.read_u32()?;

        // Simplified - full implementation would parse all variation data
        Ok(GvarTable {
            version,
            axes_array_offset,
            shared_tuples_count,
            shared_tuples_offset,
            glyph_variation_data: Vec::new(),
        })
    }
}
