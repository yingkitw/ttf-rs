use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// SVG table - SVG table
#[derive(Debug, Clone)]
pub struct SvgTable {
    pub version: u16,
    pub svg_documents: Vec<SvgDocument>,
}

#[derive(Debug, Clone)]
pub struct SvgDocument {
    pub start_glyph_id: u16,
    pub end_glyph_id: u16,
    pub svg_data: Vec<u8>,
}

impl TtfTable for SvgTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let num_svg_documents = reader.read_u32()?;

        let mut svg_documents = Vec::new();

        // Read SVG document offsets (simplified)
        let mut offsets = Vec::new();
        for _ in 0..num_svg_documents {
            offsets.push(reader.read_u32()?);
        }

        Ok(SvgTable {
            version,
            svg_documents,
        })
    }
}
