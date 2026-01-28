use crate::error::{Result, TtfError};
use crate::stream::{FontReader, FontWriter};
use crate::tables::{TtfTable, TtfTableWrite};

/// HEAD table - Font header
#[derive(Debug, Clone)]
pub struct HeadTable {
    pub table_version: f32,
    pub font_revision: f32,
    pub checksum_adjustment: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: u64,
    pub modified: u64,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}

impl HeadTable {
    pub const MAGIC_NUMBER: u32 = 0x5F0F3CF5;

    pub fn is_short_loca_format(&self) -> bool {
        self.index_to_loc_format == 0
    }

    pub fn is_long_loca_format(&self) -> bool {
        self.index_to_loc_format == 1
    }
}

impl TtfTable for HeadTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let start_pos = reader.position();

        let table_version = reader.read_fixed()?;
        let font_revision = reader.read_fixed()?;
        let checksum_adjustment = reader.read_u32()?;
        let magic_number = reader.read_u32()?;

        if magic_number != Self::MAGIC_NUMBER {
            return Err(TtfError::ParseError(format!(
                "Invalid magic number in head table: expected {:#x}, got {:#x}",
                Self::MAGIC_NUMBER, magic_number
            )));
        }

        let flags = reader.read_u16()?;
        let units_per_em = reader.read_u16()?;
        let created = reader.read_long_datetime()?;
        let modified = reader.read_long_datetime()?;
        let x_min = reader.read_i16()?;
        let y_min = reader.read_i16()?;
        let x_max = reader.read_i16()?;
        let y_max = reader.read_i16()?;
        let mac_style = reader.read_u16()?;
        let lowest_rec_ppem = reader.read_u16()?;
        let font_direction_hint = reader.read_i16()?;
        let index_to_loc_format = reader.read_i16()?;
        let glyph_data_format = reader.read_i16()?;

        // Skip any remaining bytes (shouldn't be any in a valid head table)
        let bytes_read = reader.position() - start_pos;
        if bytes_read < length as usize {
            reader.skip(length as usize - bytes_read)?;
        }

        Ok(HeadTable {
            table_version,
            font_revision,
            checksum_adjustment,
            magic_number,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        })
    }
}

impl TtfTableWrite for HeadTable {
    fn table_tag() -> &'static [u8; 4] {
        b"head"
    }

    fn write(&self, writer: &mut FontWriter) -> Result<()> {
        writer.write_fixed(self.table_version);
        writer.write_fixed(self.font_revision);
        writer.write_u32(self.checksum_adjustment);
        writer.write_u32(self.magic_number);
        writer.write_u16(self.flags);
        writer.write_u16(self.units_per_em);
        writer.write_long_datetime(self.created);
        writer.write_long_datetime(self.modified);
        writer.write_i16(self.x_min);
        writer.write_i16(self.y_min);
        writer.write_i16(self.x_max);
        writer.write_i16(self.y_max);
        writer.write_u16(self.mac_style);
        writer.write_u16(self.lowest_rec_ppem);
        writer.write_i16(self.font_direction_hint);
        writer.write_i16(self.index_to_loc_format);
        writer.write_i16(self.glyph_data_format);
        Ok(())
    }
}
