use crate::error::{Result, TtfError};
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// CMAP table - Character to glyph mapping
#[derive(Debug, Clone)]
pub struct CmapTable {
    pub version: u16,
    pub encoding_records: Vec<EncodingRecord>,
    pub subtables: Vec<CmapSubtable>,
}

#[derive(Debug, Clone)]
pub struct EncodingRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub enum CmapSubtable {
    Format0(Format0),
    Format4(Format4),
    Format6(Format6),
    Format12(Format12),
}

#[derive(Debug, Clone)]
pub struct Format0 {
    pub format: u16,
    pub length: u16,
    pub language: u16,
    pub glyph_id_array: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Format4 {
    pub format: u16,
    pub length: u16,
    pub language: u16,
    pub seg_count_x2: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub end_codes: Vec<u16>,
    pub start_codes: Vec<u16>,
    pub id_deltas: Vec<i16>,
    pub id_range_offsets: Vec<u16>,
    pub glyph_id_array: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Format6 {
    pub format: u16,
    pub length: u16,
    pub language: u16,
    pub first_code: u16,
    pub entry_count: u16,
    pub glyph_id_array: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Format12 {
    pub format: u32,
    pub length: u32,
    pub language: u32,
    pub groups: Vec<SequentialMapGroup>,
}

#[derive(Debug, Clone)]
pub struct SequentialMapGroup {
    pub start_char_code: u32,
    pub end_char_code: u32,
    pub start_glyph_code: u32,
}

impl Format0 {
    pub fn get_glyph(&self, char_code: u8) -> Option<u16> {
        Some(self.glyph_id_array[char_code as usize] as u16)
    }
}

impl Format4 {
    pub fn seg_count(&self) -> u16 {
        self.seg_count_x2 / 2
    }

    pub fn get_glyph(&self, char_code: u16) -> Option<u16> {
        let seg_count = self.seg_count();

        // Binary search for the segment
        let mut min = 0;
        let mut max = (seg_count - 1) as usize;

        loop {
            let mid = (min + max) / 2;

            if char_code > self.end_codes[mid] {
                if min == max {
                    return None;
                }
                min = mid + 1;
            } else if char_code < self.start_codes[mid] {
                if min == max {
                    return None;
                }
                max = mid - 1;
            } else {
                // Found the segment
                let start_code = self.start_codes[mid];
                let id_delta = self.id_deltas[mid] as i32;
                let id_range_offset = self.id_range_offsets[mid];

                if id_range_offset == 0 {
                    // Simple case
                    return Some((char_code as i32 + id_delta) as u16);
                } else {
                    // Complex case with id_range_offset
                    let offset_index =
                        (id_range_offset as usize / 2 + (char_code as usize - start_code as usize))
                            as usize;

                    if offset_index >= self.glyph_id_array.len() {
                        return None;
                    }

                    let glyph_id = self.glyph_id_array[offset_index];

                    if glyph_id == 0 {
                        return None;
                    }

                    return Some(glyph_id);
                }
            }
        }
    }
}

impl Format6 {
    pub fn get_glyph(&self, char_code: u16) -> Option<u16> {
        if char_code >= self.first_code {
            let index = (char_code - self.first_code) as usize;
            if index < self.glyph_id_array.len() {
                return Some(self.glyph_id_array[index]);
            }
        }
        None
    }
}

impl Format12 {
    pub fn get_glyph(&self, char_code: u32) -> Option<u32> {
        // Binary search through groups
        let mut min = 0;
        let mut max = self.groups.len() - 1;

        while min <= max {
            let mid = (min + max) / 2;
            let group = &self.groups[mid];

            if char_code < group.start_char_code {
                if mid == 0 {
                    return None;
                }
                max = mid - 1;
            } else if char_code > group.end_char_code {
                min = mid + 1;
            } else {
                // char_code is within this group
                return Some(group.start_glyph_code + (char_code - group.start_char_code));
            }
        }

        None
    }
}

impl CmapTable {
    pub fn get_best_subtable(&self) -> Option<&CmapSubtable> {
        // Priority: Unicode BMP (3,1), Unicode (0,3), Windows Symbol (3,0), Mac Roman (1,0)
        let preferred = [
            (3, 1), // Unicode BMP
            (0, 4), // Unicode 2.0+
            (0, 3), // Unicode 1.1
            (3, 0), // Symbol
            (1, 0), // Roman
        ];

        for (platform_id, encoding_id) in &preferred {
            for (i, record) in self.encoding_records.iter().enumerate() {
                if record.platform_id == *platform_id && record.encoding_id == *encoding_id {
                    return self.subtables.get(i);
                }
            }
        }

        self.subtables.first()
    }

    pub fn map_char(&self, c: char) -> Option<u32> {
        let code = c as u32;

        if let Some(subtable) = self.get_best_subtable() {
            match subtable {
                CmapSubtable::Format0(f) if code <= 0xFF => {
                    f.get_glyph(code as u8).map(|g| g as u32)
                }
                CmapSubtable::Format4(f) if code <= 0xFFFF => f.get_glyph(code as u16).map(|g| g as u32),
                CmapSubtable::Format12(f) => f.get_glyph(code),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl TtfTable for CmapTable {
    fn from_reader(reader: &mut FontReader, _length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let num_tables = reader.read_u16()?;

        let mut encoding_records = Vec::with_capacity(num_tables as usize);
        for _ in 0..num_tables {
            encoding_records.push(EncodingRecord {
                platform_id: reader.read_u16()?,
                encoding_id: reader.read_u16()?,
                offset: reader.read_u32()?,
            });
        }

        // Parse subtables
        let mut subtables = Vec::with_capacity(num_tables as usize);
        for record in &encoding_records {
            let current_pos = reader.position();
            reader.set_position(record.offset as usize)?;

            let format = reader.read_u16()?;
            let subtable = match format {
                0 => {
                    let length = reader.read_u16()?;
                    let language = reader.read_u16()?;
                    let glyph_id_array = reader.read_bytes(256)?;
                    CmapSubtable::Format0(Format0 {
                        format,
                        length,
                        language,
                        glyph_id_array,
                    })
                }
                4 => {
                    let length = reader.read_u16()?;
                    let language = reader.read_u16()?;
                    let seg_count_x2 = reader.read_u16()?;
                    let seg_count = seg_count_x2 / 2;
                    let search_range = reader.read_u16()?;
                    let entry_selector = reader.read_u16()?;
                    let range_shift = reader.read_u16()?;

                    let mut end_codes = Vec::with_capacity(seg_count as usize);
                    for _ in 0..seg_count {
                        end_codes.push(reader.read_u16()?);
                    }

                    let _reserved_pad = reader.read_u16()?;

                    let mut start_codes = Vec::with_capacity(seg_count as usize);
                    for _ in 0..seg_count {
                        start_codes.push(reader.read_u16()?);
                    }

                    let mut id_deltas = Vec::with_capacity(seg_count as usize);
                    for _ in 0..seg_count {
                        id_deltas.push(reader.read_i16()?);
                    }

                    let mut id_range_offsets = Vec::with_capacity(seg_count as usize);
                    for _ in 0..seg_count {
                        id_range_offsets.push(reader.read_u16()?);
                    }

                    let remaining = (length as usize)
                        .saturating_sub(2 + 2 + 2 + 2 + 2 + 2 + 2)
                        .saturating_sub(seg_count as usize * 2)
                        .saturating_sub(2)
                        .saturating_sub(seg_count as usize * 2)
                        .saturating_sub(seg_count as usize * 2)
                        .saturating_sub(seg_count as usize * 2);
                    let glyph_id_array_size = remaining / 2;
                    let mut glyph_id_array = Vec::with_capacity(glyph_id_array_size);
                    for _ in 0..glyph_id_array_size {
                        glyph_id_array.push(reader.read_u16()?);
                    }

                    CmapSubtable::Format4(Format4 {
                        format,
                        length,
                        language,
                        seg_count_x2,
                        search_range,
                        entry_selector,
                        range_shift,
                        end_codes,
                        start_codes,
                        id_deltas,
                        id_range_offsets,
                        glyph_id_array,
                    })
                }
                _ => {
                    return Err(TtfError::ParseError(format!(
                        "Unsupported cmap subtable format: {}",
                        format
                    )));
                }
            };

            subtables.push(subtable);
            reader.set_position(current_pos)?;
        }

        Ok(CmapTable {
            version,
            encoding_records,
            subtables,
        })
    }
}
