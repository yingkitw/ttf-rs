use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// MAXP table - Maximum profile
#[derive(Debug, Clone)]
pub struct MaxpTable {
    pub version: f32,
    pub num_glyphs: u16,
    // Version 1.0 fields
    pub max_points: Option<u16>,
    pub max_contours: Option<u16>,
    pub max_composite_points: Option<u16>,
    pub max_composite_contours: Option<u16>,
    pub max_zones: Option<u16>,
    pub max_twilight_points: Option<u16>,
    pub max_storage: Option<u16>,
    pub max_function_defs: Option<u16>,
    pub max_instruction_defs: Option<u16>,
    pub max_stack_elements: Option<u16>,
    pub max_size_of_instructions: Option<u16>,
    pub max_component_elements: Option<u16>,
    pub max_component_depth: Option<u16>,
}

impl MaxpTable {
    pub const VERSION_0_5: f32 = 0.5;
    pub const VERSION_1_0: f32 = 1.0;

    pub fn is_version_0_5(&self) -> bool {
        self.version == Self::VERSION_0_5
    }

    pub fn is_version_1_0(&self) -> bool {
        self.version == Self::VERSION_1_0
    }
}

impl TtfTable for MaxpTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_fixed()?;
        let num_glyphs = reader.read_u16()?;

        if version == Self::VERSION_0_5 {
            // Version 0.5 only has num_glyphs
            return Ok(MaxpTable {
                version,
                num_glyphs,
                max_points: None,
                max_contours: None,
                max_composite_points: None,
                max_composite_contours: None,
                max_zones: None,
                max_twilight_points: None,
                max_storage: None,
                max_function_defs: None,
                max_instruction_defs: None,
                max_stack_elements: None,
                max_size_of_instructions: None,
                max_component_elements: None,
                max_component_depth: None,
            });
        }

        // Version 1.0 has additional fields
        let max_points = Some(reader.read_u16()?);
        let max_contours = Some(reader.read_u16()?);
        let max_composite_points = Some(reader.read_u16()?);
        let max_composite_contours = Some(reader.read_u16()?);
        let max_zones = Some(reader.read_u16()?);
        let max_twilight_points = Some(reader.read_u16()?);
        let max_storage = Some(reader.read_u16()?);
        let max_function_defs = Some(reader.read_u16()?);
        let max_instruction_defs = Some(reader.read_u16()?);
        let max_stack_elements = Some(reader.read_u16()?);
        let max_size_of_instructions = Some(reader.read_u16()?);
        let max_component_elements = Some(reader.read_u16()?);
        let max_component_depth = Some(reader.read_u16()?);

        Ok(MaxpTable {
            version,
            num_glyphs,
            max_points,
            max_contours,
            max_composite_points,
            max_composite_contours,
            max_zones,
            max_twilight_points,
            max_storage,
            max_function_defs,
            max_instruction_defs,
            max_stack_elements,
            max_size_of_instructions,
            max_component_elements,
            max_component_depth,
        })
    }
}
