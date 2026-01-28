use crate::error::Result;
use crate::stream::FontReader;
use crate::tables::TtfTable;

/// Kern table - Kerning data
#[derive(Debug, Clone)]
pub struct KernTable {
    pub version: u16,
    pub subtables: Vec<KernSubtable>,
}

#[derive(Debug, Clone)]
pub enum KernSubtable {
    Format0(KernFormat0),
}

/// Format 0 - Kerning data format 0
#[derive(Debug, Clone)]
pub struct KernFormat0 {
    pub version: u16,
    pub length: u16,
    pub coverage: u16,
    pub pairs: Vec<KernPair>,
}

#[derive(Debug, Clone)]
pub struct KernPair {
    pub left: u16,
    pub right: u16,
    pub value: i16,
}

impl KernFormat0 {
    pub fn is_horizontal(&self) -> bool {
        self.coverage & 0x80 == 0
    }

    pub fn is_minimum(&self) -> bool {
        self.coverage & 0x40 == 0
    }

    pub fn is_cross_stream(&self) -> bool {
        self.coverage & 0x20 != 0
    }
}

impl TtfTable for KernTable {
    fn from_reader(reader: &mut FontReader, length: u32) -> Result<Self> {
        let version = reader.read_u16()?;
        let num_tables = reader.read_u16()?;

        let mut subtables = Vec::with_capacity(num_tables as usize);

        for _ in 0..num_tables {
            let subtable_version = reader.read_u16()?;
            let subtable_length = reader.read_u16()?;
            let coverage = reader.read_u16()?;

            let subtable = if (coverage & 0xFF) == 0 {
                // Format 0
                let num_pairs = reader.read_u16()?;
                let search_range = reader.read_u16()?;
                let entry_selector = reader.read_u16()?;
                let range_shift = reader.read_u16()?;

                let mut pairs = Vec::with_capacity(num_pairs as usize);
                for _ in 0..num_pairs {
                    pairs.push(KernPair {
                        left: reader.read_u16()?,
                        right: reader.read_u16()?,
                        value: reader.read_i16()?,
                    });
                }

                KernSubtable::Format0(KernFormat0 {
                    version: subtable_version,
                    length: subtable_length,
                    coverage,
                    pairs,
                })
            } else {
                // Skip unsupported formats
                let remaining = subtable_length.saturating_sub(8) as usize;
                reader.skip(remaining)?;
                continue;
            };

            subtables.push(subtable);
        }

        Ok(KernTable {
            version,
            subtables,
        })
    }
}
