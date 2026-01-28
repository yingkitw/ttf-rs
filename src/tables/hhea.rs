use crate::error::Result;
use crate::stream::{FontReader, FontWriter};
use crate::tables::{TtfTable, TtfTableWrite};

/// HHEA table - Horizontal header
#[derive(Debug, Clone)]
pub struct HheaTable {
    pub table_version: f32,
    pub ascent: i16,
    pub descent: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub reserved0: i16,
    pub reserved1: i16,
    pub reserved2: i16,
    pub reserved3: i16,
    pub reserved4: i16,
    pub metric_data_format: i16,
    pub number_of_h_metrics: u16,
}

impl HheaTable {
    pub fn get_line_height(&self) -> i32 {
        self.ascent as i32 - self.descent as i32 + self.line_gap as i32
    }

    /// Alias for ascent for compatibility with modifier code
    pub fn ascender(&self) -> i16 {
        self.ascent
    }

    /// Alias for descent for compatibility with modifier code
    pub fn descender(&self) -> i16 {
        self.descent
    }
}

impl TtfTable for HheaTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let table_version = reader.read_fixed()?;
        let ascent = reader.read_i16()?;
        let descent = reader.read_i16()?;
        let line_gap = reader.read_i16()?;
        let advance_width_max = reader.read_u16()?;
        let min_left_side_bearing = reader.read_i16()?;
        let min_right_side_bearing = reader.read_i16()?;
        let x_max_extent = reader.read_i16()?;
        let caret_slope_rise = reader.read_i16()?;
        let caret_slope_run = reader.read_i16()?;
        let caret_offset = reader.read_i16()?;
        let reserved0 = reader.read_i16()?;
        let reserved1 = reader.read_i16()?;
        let reserved2 = reader.read_i16()?;
        let reserved3 = reader.read_i16()?;
        let reserved4 = reader.read_i16()?;
        let metric_data_format = reader.read_i16()?;
        let number_of_h_metrics = reader.read_u16()?;

        // Check if we've read the expected amount (hhea is always 36 bytes)
        let bytes_read = reader.position();
        if bytes_read < length as usize {
            reader.skip(length as usize - bytes_read)?;
        }

        Ok(HheaTable {
            table_version,
            ascent,
            descent,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            reserved0,
            reserved1,
            reserved2,
            reserved3,
            reserved4,
            metric_data_format,
            number_of_h_metrics,
        })
    }
}

impl TtfTableWrite for HheaTable {
    fn table_tag() -> &'static [u8; 4] {
        b"hhea"
    }

    fn write(&self, writer: &mut FontWriter) -> Result<()> {
        writer.write_fixed(self.table_version);
        writer.write_i16(self.ascent);
        writer.write_i16(self.descent);
        writer.write_i16(self.line_gap);
        writer.write_u16(self.advance_width_max);
        writer.write_i16(self.min_left_side_bearing);
        writer.write_i16(self.min_right_side_bearing);
        writer.write_i16(self.x_max_extent);
        writer.write_i16(self.caret_slope_rise);
        writer.write_i16(self.caret_slope_run);
        writer.write_i16(self.caret_offset);
        writer.write_i16(self.reserved0);
        writer.write_i16(self.reserved1);
        writer.write_i16(self.reserved2);
        writer.write_i16(self.reserved3);
        writer.write_i16(self.reserved4);
        writer.write_i16(self.metric_data_format);
        writer.write_u16(self.number_of_h_metrics);
        Ok(())
    }
}
