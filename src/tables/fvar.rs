use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// FVAR table - Font variations
#[derive(Debug, Clone)]
pub struct FvarTable {
    pub version: u16,
    pub offset_to_axes: u16,
    pub count_size_pairs: u16,
    pub axes: Vec<Axis>,
    pub instance_size: u16,
    pub count_instances: u16,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Clone)]
pub struct Axis {
    pub axis_tag: [u8; 4],
    pub axis_name_id: u16,
    pub min_value: f32,
    pub default_value: f32,
    pub max_value: f32,
    pub flags: u16,
    pub axis_name_id_short: u16,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub subfamily_name_id: u16,
    pub flags: u16,
    pub coordinates: Vec<f32>,
    pub post_script_name_id: Option<u16>,
}

impl TtfTable for FvarTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let offset_to_axes = reader.read_u16()?;
        let count_size_pairs = reader.read_u16()?;

        // Skip reserved
        reader.skip(2)?;

        let mut axes = Vec::new();
        let current_offset = reader.position();

        for _ in 0..count_size_pairs {
            reader.set_position(current_offset + offset_to_axes as usize)?;

            let axis_tag = reader.read_tag()?;
            let axis_name_id = reader.read_u16()?;
            let min_value = reader.read_fixed()?;
            let default_value = reader.read_fixed()?;
            let max_value = reader.read_fixed()?;
            let flags = reader.read_u16()?;
            let axis_name_id_short = reader.read_u16()?;

            axes.push(Axis {
                axis_tag,
                axis_name_id,
                min_value,
                default_value,
                max_value,
                flags,
                axis_name_id_short,
            });

            break; // Simplified - full implementation would iterate all axes
        }

        // Read instance data (simplified)
        let instance_size = reader.read_u16()?;
        let count_instances = reader.read_u16()?;

        Ok(FvarTable {
            version,
            offset_to_axes,
            count_size_pairs,
            axes,
            instance_size,
            count_instances,
            instances: Vec::new(),
        })
    }
}
