use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// POST table - PostScript information
#[derive(Debug, Clone)]
pub struct PostTable {
    pub format: f32,
    pub italic_angle: f32,
    pub underline_position: i16,
    pub underline_thickness: i16,
    pub is_fixed_pitch: u32,
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
}

impl PostTable {
    pub const VERSION_1_0: f32 = 1.0;
    pub const VERSION_2_0: f32 = 2.0;
    pub const VERSION_3_0: f32 = 3.0;
    pub const VERSION_4_0: f32 = 4.0;
}

impl TtfTable for PostTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let format = reader.read_fixed()?;
        let italic_angle = reader.read_fixed()?;
        let underline_position = reader.read_i16()?;
        let underline_thickness = reader.read_i16()?;
        let is_fixed_pitch = reader.read_u32()?;
        let min_mem_type42 = reader.read_u32()?;
        let max_mem_type42 = reader.read_u32()?;
        let min_mem_type1 = reader.read_u32()?;
        let max_mem_type1 = reader.read_u32()?;

        // Additional fields for format 2.0 and 3.0 would be read here
        // For simplicity, we skip them in this basic implementation

        let bytes_read = reader.position();
        if bytes_read < length as usize {
            reader.skip(length as usize - bytes_read)?;
        }

        Ok(PostTable {
            format,
            italic_angle,
            underline_position,
            underline_thickness,
            is_fixed_pitch,
            min_mem_type42,
            max_mem_type42,
            min_mem_type1,
            max_mem_type1,
        })
    }
}
