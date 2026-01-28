use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// COLR table - Color table
#[derive(Debug, Clone)]
pub struct ColrTable {
    pub version: u16,
    pub num_base_glyph_records: u32,
    pub num_layer_records: u32,
    pub num_clip_boxes: u32,
    pub base_glyph_records: Vec<BaseGlyphRecord>,
    pub layer_records: Vec<LayerRecord>,
    pub clip_boxes: Vec<ClipBox>,
}

#[derive(Debug, Clone)]
pub struct BaseGlyphRecord {
    pub glyph_id: u16,
    pub first_layer_index: u16,
    pub num_layers: u16,
}

#[derive(Debug, Clone)]
pub struct LayerRecord {
    pub glyph_id: u16,
    pub palette_entry_index: u16,
}

#[derive(Debug, Clone)]
pub struct ClipBox {
    pub format: u8,
    pub clip_box: [i32; 4], // xMin, yMin, xMax, yMax
}

impl TtfTable for ColrTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let num_base_glyph_records = reader.read_u32()?;
        let num_layer_records = reader.read_u32()?;
        let num_clip_boxes = reader.read_u32()?;

        let mut base_glyph_records = Vec::new();
        let mut layer_records = Vec::new();
        let mut clip_boxes = Vec::new();

        // Simplified - full implementation would parse all records
        Ok(ColrTable {
            version,
            num_base_glyph_records,
            num_layer_records,
            num_clip_boxes,
            base_glyph_records,
            layer_records,
            clip_boxes,
        })
    }
}
