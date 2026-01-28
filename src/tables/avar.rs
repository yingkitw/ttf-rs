use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// AVAR table - Axis variations
#[derive(Debug, Clone)]
pub struct AvarTable {
    pub version: u16,
    pub reserved: u16,
    pub axis_segment_maps: Vec<AxisSegmentMap>,
}

#[derive(Debug, Clone)]
pub struct AxisSegmentMap {
    pub from_axis_count: u16,
    pub from_coordinate_array: Vec<f32>,
    pub to_axis_count: u16,
    pub to_coordinate_array: Vec<f32>,
}

impl TtfTable for AvarTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let reserved = reader.read_u16()?;
        let axis_count = reader.read_u16()?;

        let mut axis_segment_maps = Vec::new();

        for _ in 0..axis_count {
            let from_axis_count = reader.read_u16()?;
            let mut from_coordinate_array = Vec::new();
            for _ in 0..from_axis_count {
                from_coordinate_array.push(reader.read_fixed()?);
            }

            let to_axis_count = reader.read_u16()?;
            let mut to_coordinate_array = Vec::new();
            for _ in 0..to_axis_count {
                to_coordinate_array.push(reader.read_fixed()?);
            }

            axis_segment_maps.push(AxisSegmentMap {
                from_axis_count,
                from_coordinate_array,
                to_axis_count,
                to_coordinate_array,
            });
        }

        Ok(AvarTable {
            version,
            reserved,
            axis_segment_maps,
        })
    }
}
