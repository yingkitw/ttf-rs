use crate::error::Result;
use crate::stream::FontReader;
use super::loca::LocaTable;

/// 2D Point for glyph coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Apply transformation matrix to the point
    pub fn transform(&self, transform: &Transform) -> Point {
        Point {
            x: transform.xx * self.x + transform.xy * self.y + transform.dx,
            y: transform.yx * self.x + transform.yy * self.y + transform.dy,
        }
    }
}

/// Bounding box for glyphs
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

impl BoundingBox {
    pub fn new(x_min: f32, y_min: f32, x_max: f32, y_max: f32) -> Self {
        Self {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    /// Create from existing glyph bounding box values
    pub fn from_glyph(x_min: i16, y_min: i16, x_max: i16, y_max: i16) -> Self {
        Self {
            x_min: x_min as f32,
            y_min: y_min as f32,
            x_max: x_max as f32,
            y_max: y_max as f32,
        }
    }

    /// Calculate width of bounding box
    pub fn width(&self) -> f32 {
        self.x_max - self.x_min
    }

    /// Calculate height of bounding box
    pub fn height(&self) -> f32 {
        self.y_max - self.y_min
    }

    /// Merge two bounding boxes
    pub fn merge(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            x_min: self.x_min.min(other.x_min),
            y_min: self.y_min.min(other.y_min),
            x_max: self.x_max.max(other.x_max),
            y_max: self.y_max.max(other.y_max),
        }
    }
}

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

    /// Calculate the bounding box for this glyph
    pub fn calculate_bounding_box(&self) -> Option<BoundingBox> {
        if self.is_empty() {
            return None;
        }

        // For simple glyphs, calculate from points
        if let GlyphData::Simple(simple) = &self.data {
            if simple.x_coordinates.is_empty() {
                return Some(BoundingBox::from_glyph(
                    self.x_min, self.y_min, self.x_max, self.y_max
                ));
            }

            let mut x_min = f32::MAX;
            let mut y_min = f32::MAX;
            let mut x_max = f32::MIN;
            let mut y_max = f32::MIN;

            for i in 0..simple.x_coordinates.len() {
                let x = simple.x_coordinates[i] as f32;
                let y = simple.y_coordinates[i] as f32;
                x_min = x_min.min(x);
                y_min = y_min.min(y);
                x_max = x_max.max(x);
                y_max = y_max.max(y);
            }

            Some(BoundingBox::new(x_min, y_min, x_max, y_max))
        } else if let GlyphData::Composite(composite) = &self.data {
            // For composite glyphs, return the stored bounding box
            Some(BoundingBox::from_glyph(
                self.x_min, self.y_min, self.x_max, self.y_max
            ))
        } else {
            None
        }
    }

    /// Transform the glyph by applying a transformation matrix
    pub fn transform(&mut self, transform: &Transform) -> Result<()> {
        match &mut self.data {
            GlyphData::Simple(simple) => {
                // Transform all coordinates
                for (x, y) in simple.x_coordinates.iter_mut().zip(simple.y_coordinates.iter_mut()) {
                    let old_x = *x as f32;
                    let old_y = *y as f32;
                    let new_x = transform.xx * old_x + transform.xy * old_y + transform.dx;
                    let new_y = transform.yx * old_x + transform.yy * old_y + transform.dy;
                    *x = new_x.round() as i16;
                    *y = new_y.round() as i16;
                }

                // Update bounding box
                if let Some(bbox) = self.calculate_bounding_box() {
                    self.x_min = bbox.x_min.round() as i16;
                    self.y_min = bbox.y_min.round() as i16;
                    self.x_max = bbox.x_max.round() as i16;
                    self.y_max = bbox.y_max.round() as i16;
                }
            }
            GlyphData::Composite(composite) => {
                // Update the transform for each component
                for component in &mut composite.components {
                    component.transform = Self::combine_transforms(&component.transform, transform);
                }

                // Update bounding box
                if let Some(bbox) = self.calculate_bounding_box() {
                    self.x_min = bbox.x_min.round() as i16;
                    self.y_min = bbox.y_min.round() as i16;
                    self.x_max = bbox.x_max.round() as i16;
                    self.y_max = bbox.y_max.round() as i16;
                }
            }
            GlyphData::Empty => {}
        }
        Ok(())
    }

    /// Scale the glyph by the given factors
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) -> Result<()> {
        let transform = Transform {
            xx: scale_x,
            xy: 0.0,
            yx: 0.0,
            yy: scale_y,
            dx: 0.0,
            dy: 0.0,
        };
        self.transform(&transform)
    }

    /// Rotate the glyph by the given angle (in radians)
    pub fn rotate(&mut self, angle: f32) -> Result<()> {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let transform = Transform {
            xx: cos_a,
            xy: -sin_a,
            yx: sin_a,
            yy: cos_a,
            dx: 0.0,
            dy: 0.0,
        };
        self.transform(&transform)
    }

    /// Translate the glyph by the given offsets
    pub fn translate(&mut self, dx: f32, dy: f32) -> Result<()> {
        let transform = Transform {
            xx: 1.0,
            xy: 0.0,
            yx: 0.0,
            yy: 1.0,
            dx,
            dy,
        };
        self.transform(&transform)
    }

    /// Combine two transforms (applied right to left: second then first)
    fn combine_transforms(first: &Transform, second: &Transform) -> Transform {
        Transform {
            xx: first.xx * second.xx + first.xy * second.yx,
            xy: first.xx * second.xy + first.xy * second.yy,
            yx: first.yx * second.xx + first.yy * second.yx,
            yy: first.yx * second.xy + first.yy * second.yy,
            dx: first.xx * second.dx + first.xy * second.dy + first.dx,
            dy: first.yx * second.dx + first.yy * second.dy + first.dy,
        }
    }

    /// Simplify the glyph outline by removing redundant points
    /// This is a basic implementation that removes collinear points
    pub fn simplify(&mut self, tolerance: f32) -> Result<()> {
        if let GlyphData::Simple(simple) = &mut self.data {
            if simple.x_coordinates.len() < 3 {
                return Ok(());
            }

            let mut to_remove = vec![false; simple.x_coordinates.len()];

            // Check each point (except endpoints of contours)
            let mut contour_start = 0;
            for &end_pt in &simple.end_pts_of_contours {
                let end = end_pt as usize;

                // Check interior points of this contour
                for i in (contour_start + 1)..end {
                    let prev = if i > contour_start { i - 1 } else { end };
                    let next = if i < end { i + 1 } else { contour_start };

                    let x1 = simple.x_coordinates[prev] as f32;
                    let y1 = simple.y_coordinates[prev] as f32;
                    let x2 = simple.x_coordinates[i] as f32;
                    let y2 = simple.y_coordinates[i] as f32;
                    let x3 = simple.x_coordinates[next] as f32;
                    let y3 = simple.y_coordinates[next] as f32;

                    // Check if point i is collinear with prev and next
                    let area = ((x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1)).abs();

                    if area < tolerance {
                        // Also check distance to ensure point isn't adding precision
                        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                        if dist < tolerance || ((x3 - x2).powi(2) + (y3 - y2).powi(2)).sqrt() < tolerance {
                            to_remove[i] = true;
                        }
                    }
                }

                contour_start = end + 1;
            }

            // Remove marked points (in reverse order to preserve indices)
            for i in (0..to_remove.len()).rev() {
                if to_remove[i] {
                    simple.x_coordinates.remove(i);
                    simple.y_coordinates.remove(i);
                    simple.flags.remove(i);

                    // Update contour end indices
                    for end_pt in &mut simple.end_pts_of_contours {
                        if *end_pt as usize > i {
                            *end_pt = (*end_pt as usize - 1) as u16;
                        }
                    }
                }
            }
        }
        Ok(())
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

    pub fn get_glyph_mut(&mut self, index: usize) -> Option<&mut Glyph> {
        self.glyphs.get_mut(index)
    }

    /// Resolve a composite glyph by flattening it into a simple glyph
    /// This resolves all component references and transforms
    pub fn resolve_composite(&self, glyph_index: usize) -> Result<Option<Glyph>> {
        let glyph = self.get_glyph(glyph_index);
        let glyph = match glyph {
            Some(g) => g,
            None => return Ok(None),
        };

        match &glyph.data {
            GlyphData::Simple(_) | GlyphData::Empty => {
                // Already simple or empty, return a clone
                Ok(Some(glyph.clone()))
            }
            GlyphData::Composite(composite) => {
                // Resolve all components
                let mut all_points: Vec<(i16, i16)> = Vec::new();
                let mut all_flags: Vec<u8> = Vec::new();
                let mut all_contours: Vec<u16> = Vec::new();
                let mut total_contours = 0;

                for component in &composite.components {
                    let component_glyph = self.get_glyph(component.glyph_index as usize);
                    let component_glyph = match component_glyph {
                        Some(g) => g,
                        None => continue,
                    };

                    if let GlyphData::Simple(simple) = &component_glyph.data {
                        let transform = &component.transform;

                        // Transform and collect all points from this component
                        let point_count = simple.x_coordinates.len();
                        for i in 0..point_count {
                            let x = simple.x_coordinates[i] as f32;
                            let y = simple.y_coordinates[i] as f32;

                            let new_x = transform.xx * x + transform.xy * y + transform.dx;
                            let new_y = transform.yx * x + transform.yy * y + transform.dy;

                            all_points.push((new_x.round() as i16, new_y.round() as i16));
                            all_flags.push(simple.flags.get(i).copied().unwrap_or(0));
                        }

                        // Update contour endpoints
                        for end_pt in &simple.end_pts_of_contours {
                            all_contours.push((*end_pt as usize + total_contours) as u16);
                            total_contours += *end_pt as usize + 1;
                        }
                    }
                }

                // Build a new simple glyph from resolved components
                if !all_points.is_empty() {
                    let mut x_coordinates = Vec::new();
                    let mut y_coordinates = Vec::new();
                    for (x, y) in &all_points {
                        x_coordinates.push(*x);
                        y_coordinates.push(*y);
                    }

                    // Calculate bounding box
                    let x_min = x_coordinates.iter().min().unwrap();
                    let y_min = y_coordinates.iter().min().unwrap();
                    let x_max = x_coordinates.iter().max().unwrap();
                    let y_max = y_coordinates.iter().max().unwrap();

                    Ok(Some(Glyph {
                        number_of_contours: all_contours.len() as i16,
                        x_min: *x_min,
                        y_min: *y_min,
                        x_max: *x_max,
                        y_max: *y_max,
                        data: GlyphData::Simple(SimpleGlyph {
                            end_pts_of_contours: all_contours,
                            instruction_length: 0,
                            instructions: Vec::new(),
                            flags: all_flags,
                            x_coordinates,
                            y_coordinates,
                        }),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
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
