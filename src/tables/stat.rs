use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// STAT table - Style attributes
#[derive(Debug, Clone)]
pub struct StatTable {
    pub version: u32,
    pub design_axis_count: u16,
    pub design_axes: Vec<DesignAxis>,
    pub axis_value_count: u16,
    pub axis_values: Vec<AxisValue>,
}

#[derive(Debug, Clone)]
pub struct DesignAxis {
    pub axis_tag: [u8; 4],
    pub axis_name_id: u16,
    pub axis_ordering: u16,
}

#[derive(Debug, Clone)]
pub struct AxisValue {
    pub format: u16,
    pub axis_index: u16,
    pub value: Option<u16>,
    pub name_id: Option<u16>,
}

impl TtfTable for StatTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u32()?;
        let design_axis_count = reader.read_u16()?;

        let mut design_axes = Vec::new();
        for _ in 0..design_axis_count {
            design_axes.push(DesignAxis {
                axis_tag: reader.read_tag()?,
                axis_name_id: reader.read_u16()?,
                axis_ordering: reader.read_u16()?,
            });
        }

        let axis_value_count = reader.read_u16()?;

        // Simplified - full implementation would parse all axis values
        Ok(StatTable {
            version,
            design_axis_count,
            design_axes,
            axis_value_count,
            axis_values: Vec::new(),
        })
    }
}
