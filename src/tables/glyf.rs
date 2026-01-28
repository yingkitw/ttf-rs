use crate::error::Result;
use crate::stream::FontReader;
use super::loca::LocaTable;

/// GLYF table - Glyph data
#[derive(Debug, Clone)]
pub struct GlyfTable {
    pub glyphs: Vec<Glyph>,
}

#[derive(Debug, Clone)]
pub struct Glyph {
    pub number_of_contours: i16,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub data: GlyphData,
}

#[derive(Debug, Clone)]
pub enum GlyphData {
    Simple(SimpleGlyph),
    Composite(CompositeGlyph),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SimpleGlyph {
    pub end_pts_of_contours: Vec<u16>,
    pub instruction_length: u16,
    pub instructions: Vec<u8>,
    pub flags: Vec<u8>,
    pub x_coordinates: Vec<i16>,
    pub y_coordinates: Vec<i16>,
}

#[derive(Debug, Clone)]
pub struct CompositeGlyph {
    pub components: Vec<GlyphComponent>,
}

#[derive(Debug, Clone)]
pub struct GlyphComponent {
    pub flags: u16,
    pub glyph_index: u16,
    pub arg1: i16,
    pub arg2: i16,
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub xx: f32,
    pub xy: f32,
    pub yx: f32,
    pub yy: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Glyph {
    pub fn is_simple(&self) -> bool {
        matches!(self.data, GlyphData::Simple(_))
    }

    pub fn is_composite(&self) -> bool {
        matches!(self.data, GlyphData::Composite(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.data, GlyphData::Empty)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            xx: 1.0,
            xy: 0.0,
            yx: 0.0,
            yy: 1.0,
            dx: 0.0,
            dy: 0.0,
        }
    }
}

impl GlyfTable {
    pub fn from_reader(reader: &mut FontReader, _length: u32, loca: &LocaTable, num_glyphs: u16) -> Result<Self> {
        let mut glyphs = Vec::with_capacity(num_glyphs as usize);

        for i in 0..num_glyphs {
            let offset = loca.get_offset(i as usize)?;
            let next_offset = loca.get_offset(i as usize + 1)?;

            if offset == next_offset {
                // Empty glyph
                glyphs.push(Glyph {
                    number_of_contours: 0,
                    x_min: 0,
                    y_min: 0,
                    x_max: 0,
                    y_max: 0,
                    data: GlyphData::Empty,
                });
                continue;
            }

            reader.set_position(offset as usize)?;

            let number_of_contours = reader.read_i16()?;
            let x_min = reader.read_i16()?;
            let y_min = reader.read_i16()?;
            let x_max = reader.read_i16()?;
            let y_max = reader.read_i16()?;

            let data = if number_of_contours > 0 {
                // Simple glyph
                GlyphData::Simple(SimpleGlyph::read(reader, number_of_contours as usize)?)
            } else if number_of_contours < 0 {
                // Composite glyph
                GlyphData::Composite(CompositeGlyph::read(reader)?)
            } else {
                GlyphData::Empty
            };

            glyphs.push(Glyph {
                number_of_contours,
                x_min,
                y_min,
                x_max,
                y_max,
                data,
            });
        }

        Ok(GlyfTable { glyphs })
    }

    pub fn get_glyph(&self, index: usize) -> Option<&Glyph> {
        self.glyphs.get(index)
    }
}

impl SimpleGlyph {
    fn read(reader: &mut FontReader, num_contours: usize) -> Result<Self> {
        let mut end_pts_of_contours = Vec::with_capacity(num_contours);
        for _ in 0..num_contours {
            end_pts_of_contours.push(reader.read_u16()?);
        }

        let instruction_length = reader.read_u16()?;
        let mut instructions = Vec::with_capacity(instruction_length as usize);
        for _ in 0..instruction_length {
            instructions.push(reader.read_u8()?);
        }

        let num_points = if let Some(&last) = end_pts_of_contours.last() {
            last as usize + 1
        } else {
            0
        };

        let mut flags = Vec::with_capacity(num_points);
        let mut i = 0;
        while i < num_points {
            let flag = reader.read_u8()?;
            flags.push(flag);
            i += 1;

            // Check for repeat flag
            if flag & 0x8 != 0 {
                let repeat_count = reader.read_u8()? as usize;
                for _ in 0..repeat_count {
                    flags.push(flag);
                    i += 1;
                }
            }
        }

        let mut x_coordinates = Vec::with_capacity(num_points);
        let mut x = 0i16;
        for &flag in &flags {
            if flag & 0x2 != 0 {
                // 1-byte signed
                let val = reader.read_i8()?;
                x += if flag & 0x10 != 0 { val as i16 } else { -(val as i16) };
            } else if flag & 0x10 == 0 {
                // 2-byte signed
                x += reader.read_i16()?;
            }
            // else: x is unchanged (duplicate)
            x_coordinates.push(x);
        }

        let mut y_coordinates = Vec::with_capacity(num_points);
        let mut y = 0i16;
        for &flag in &flags {
            if flag & 0x4 != 0 {
                // 1-byte signed
                let val = reader.read_i8()?;
                y += if flag & 0x20 != 0 { val as i16 } else { -(val as i16) };
            } else if flag & 0x20 == 0 {
                // 2-byte signed
                y += reader.read_i16()?;
            }
            // else: y is unchanged (duplicate)
            y_coordinates.push(y);
        }

        Ok(SimpleGlyph {
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags,
            x_coordinates,
            y_coordinates,
        })
    }
}

impl CompositeGlyph {
    fn read(reader: &mut FontReader) -> Result<Self> {
        let mut components = Vec::new();

        loop {
            let flags = reader.read_u16()?;
            let glyph_index = reader.read_u16()?;

            let arg1 = if flags & 0x1 != 0 {
                reader.read_i16()?
            } else {
                reader.read_i8()? as i16
            };

            let arg2 = if flags & 0x1 != 0 {
                reader.read_i16()?
            } else {
                reader.read_i8()? as i16
            };

            let mut transform = Transform::default();

            // Read transform based on flags
            transform.xx = if flags & 0x80 != 0 {
                reader.read_f2dot14()?
            } else {
                1.0
            };

            transform.xy = if flags & 0x40 != 0 {
                reader.read_f2dot14()?
            } else {
                0.0
            };

            transform.yx = if flags & 0x20 != 0 {
                reader.read_f2dot14()?
            } else {
                0.0
            };

            transform.yy = if flags & 0x10 != 0 {
                reader.read_f2dot14()?
            } else {
                1.0
            };

            if flags & 0x8 != 0 {
                transform.dx = reader.read_i16()? as f32;
                transform.dy = reader.read_i16()? as f32;
            } else {
                // Use arg1/arg2 as offsets
                transform.dx = arg1 as f32;
                transform.dy = arg2 as f32;
            }

            components.push(GlyphComponent {
                flags,
                glyph_index,
                arg1,
                arg2,
                transform,
            });

            if flags & 0x20 == 0 {
                // More components flag
                break;
            }
        }

        Ok(CompositeGlyph { components })
    }
}
