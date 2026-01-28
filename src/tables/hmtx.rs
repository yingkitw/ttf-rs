use crate::error::Result;
use crate::stream::{FontReader, FontWriter};
use crate::tables::TtfTableWrite;

/// HMTX table - Horizontal metrics
#[derive(Debug, Clone)]
pub struct HmtxTable {
    pub h_metrics: Vec<LongHorMetric>,
    pub left_side_bearings: Vec<i16>,
}

#[derive(Debug, Clone)]
pub struct LongHorMetric {
    pub advance_width: u16,
    pub lsb: i16,
}

impl HmtxTable {
    pub fn from_reader(reader: &mut FontReader, _length: u32, num_glyphs: u16, num_h_metrics: u16) -> Result<Self> {
        let mut h_metrics = Vec::with_capacity(num_h_metrics as usize);
        for _ in 0..num_h_metrics {
            h_metrics.push(LongHorMetric {
                advance_width: reader.read_u16()?,
                lsb: reader.read_i16()?,
            });
        }

        let num_lsb = num_glyphs.saturating_sub(num_h_metrics) as usize;
        let mut left_side_bearings = Vec::with_capacity(num_lsb);
        for _ in 0..num_lsb {
            left_side_bearings.push(reader.read_i16()?);
        }

        Ok(HmtxTable {
            h_metrics,
            left_side_bearings,
        })
    }

    pub fn get_advance_width(&self, glyph_index: u16) -> u16 {
        if glyph_index < self.h_metrics.len() as u16 {
            self.h_metrics[glyph_index as usize].advance_width
        } else if !self.h_metrics.is_empty() {
            self.h_metrics.last().unwrap().advance_width
        } else {
            0
        }
    }

    pub fn get_lsb(&self, glyph_index: u16) -> i16 {
        if glyph_index < self.h_metrics.len() as u16 {
            self.h_metrics[glyph_index as usize].lsb
        } else {
            let idx = glyph_index as usize - self.h_metrics.len();
            if idx < self.left_side_bearings.len() {
                self.left_side_bearings[idx]
            } else {
                0
            }
        }
    }

    /// Get all advance widths as a vector (for compatibility with modifier)
    pub fn advance_widths(&self) -> Vec<u16> {
        self.h_metrics.iter().map(|m| m.advance_width).collect()
    }
}

impl TtfTableWrite for HmtxTable {
    fn table_tag() -> &'static [u8; 4] {
        b"hmtx"
    }

    fn write(&self, writer: &mut FontWriter) -> Result<()> {
        // Write h_metrics
        for metric in &self.h_metrics {
            writer.write_u16(metric.advance_width);
            writer.write_i16(metric.lsb);
        }

        // Write left_side_bearings
        for &lsb in &self.left_side_bearings {
            writer.write_i16(lsb);
        }

        Ok(())
    }
}
