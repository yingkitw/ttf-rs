use crate::error::Result;
use crate::font::Font;
use crate::tables::glyf::{Glyph, GlyphData, SimpleGlyph};
use std::collections::HashMap;

/// Rasterizer for converting TTF outlines to bitmaps
pub struct Rasterizer {
    font: Font,
    cache: HashMap<u32, RasterizedGlyph>,
}

#[derive(Debug, Clone)]
pub struct RasterizedGlyph {
    pub glyph_id: u32,
    pub bitmap: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub advance_width: u16,
    pub left_side_bearing: i16,
}

impl Rasterizer {
    pub fn new(font: Font) -> Self {
        Self {
            font,
            cache: HashMap::new(),
        }
    }

    /// Rasterize a glyph at a specific size
    pub fn rasterize_glyph(&mut self, glyph_id: u32, size: u32) -> Result<RasterizedGlyph> {
        // Check cache first
        if let Some(cached) = self.cache.get(&glyph_id) {
            return Ok(cached.clone());
        }

        // Get the glyf table
        let glyf_table = self.font.glyf_table()?;
        let hmtx_table = self.font.hmtx_table()?;
        let head_table = self.font.head_table()?;
        let units_per_em = head_table.units_per_em as f32;

        // Get the glyph
        let glyph = glyf_table.get_glyph(glyph_id as usize)
            .ok_or_else(|| crate::error::TtfError::InvalidGlyphIndex(glyph_id as u16))?;

        // Get metrics
        let advance_width = hmtx_table.get_advance_width(glyph_id as u16);
        let lsb = hmtx_table.get_lsb(glyph_id as u16);

        // Calculate scale factor
        let scale = size as f32 / units_per_em;

        // Create bitmap
        let (bitmap, width, height) = if let GlyphData::Simple(simple) = &glyph.data {
            self.rasterize_simple(simple, scale, &glyph)?
        } else {
            // For composite or empty glyphs, create empty bitmap
            (vec![0u8; 0], 0, 0)
        };

        let rasterized = RasterizedGlyph {
            glyph_id,
            bitmap,
            width,
            height,
            advance_width,
            left_side_bearing: lsb,
        };

        // Cache the result
        self.cache.insert(glyph_id, rasterized.clone());

        Ok(rasterized)
    }

    /// Rasterize a simple glyph
    fn rasterize_simple(&self, glyph: &SimpleGlyph, scale: f32, bounds: &Glyph) -> Result<(Vec<u8>, usize, usize)> {
        if glyph.end_pts_of_contours.is_empty() || glyph.x_coordinates.is_empty() {
            return Ok((vec![0u8; 0], 0, 0));
        }

        // Calculate bounding box in pixels
        let x_min = (bounds.x_min as f32 * scale).floor() as i32;
        let y_min = (bounds.y_min as f32 * scale).floor() as i32;
        let x_max = (bounds.x_max as f32 * scale).ceil() as i32;
        let y_max = (bounds.y_max as f32 * scale).ceil() as i32;

        let width = (x_max - x_min).max(1) as usize;
        let height = (y_max - y_min).max(1) as usize;

        let mut bitmap = vec![0u8; width * height];

        // Rasterize each contour
        let mut point_idx = 0;
        for &end_pt in &glyph.end_pts_of_contours {
            let end = end_pt as usize;

            while point_idx <= end {
                if point_idx >= glyph.x_coordinates.len() {
                    break;
                }

                // Get current and next point
                let x1 = glyph.x_coordinates[point_idx];
                let y1 = glyph.y_coordinates[point_idx];

                let next_idx = if point_idx < end {
                    point_idx + 1
                } else {
                    0
                };

                let x2 = glyph.x_coordinates[next_idx];
                let y2 = glyph.y_coordinates[next_idx];

                // Convert to pixel coordinates
                let px1 = ((x1 as f32 * scale) - x_min as f32).round() as i32;
                let py1 = ((y1 as f32 * scale) - y_min as f32).round() as i32;
                let px2 = ((x2 as f32 * scale) - x_min as f32).round() as i32;
                let py2 = ((y2 as f32 * scale) - y_min as f32).round() as i32;

                // Draw line
                self.draw_line(&mut bitmap, width, height, px1, py1, px2, py2);

                point_idx += 1;
            }
        }

        Ok((bitmap, width, height))
    }

    /// Draw a line on the bitmap using Bresenham's algorithm
    fn draw_line(&self, bitmap: &mut [u8], width: usize, height: usize, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let mut x1 = x1;
        let mut y1 = y1;

        // Ensure coordinates are within bounds
        x0 = x0.max(0).min(width as i32 - 1);
        y0 = y0.max(0).min(height as i32 - 1);
        x1 = x1.max(0).min(width as i32 - 1);
        y1 = y1.max(0).min(height as i32 - 1);

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        if dx > dy {
            let mut err = dx - 2 * dy;
            while x0 != x1 {
                // Set pixel if in bounds
                let idx = (y0 as usize * width + x0 as usize);
                if idx < bitmap.len() {
                    bitmap[idx] = 255;
                }

                if err > 0 {
                    y0 += sy;
                    err -= 2 * dx;
                }
                if err < 0 {
                    err += 2 * dy;
                }
                x0 += sx;
            }
        } else {
            let mut err = dy - 2 * dx;
            while y0 != y1 {
                // Set pixel if in bounds
                let idx = (y0 as usize * width + x0 as usize);
                if idx < bitmap.len() {
                    bitmap[idx] = 255;
                }

                if err > 0 {
                    x0 += sx;
                    err -= 2 * dy;
                }
                if err < 0 {
                    err += 2 * dx;
                }
                y0 += sy;
            }
        }

        // Set final pixel
        let idx = (y1 as usize * width + x1 as usize);
        if idx < bitmap.len() {
            bitmap[idx] = 255;
        }
    }

    /// Clear the rasterization cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Font {
    /// Create a rasterizer for this font
    pub fn rasterizer(&self) -> Rasterizer {
        Rasterizer::new(self.clone())
    }
}
