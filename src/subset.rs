use crate::error::{Result, TtfError};
use crate::font::Font;
use crate::stream::FontWriter;
use crate::tables::glyf::GlyphData;
use std::collections::{HashMap, HashSet};

/// FontSubset allows creating a subset of a font with only specified glyphs
pub struct FontSubset {
    font: Font,
    glyph_ids: HashSet<u32>,
    retain_tables: HashSet<[u8; 4]>,
}

impl FontSubset {
    /// Create a new subset builder
    pub fn new(font: Font) -> Self {
        Self {
            font,
            glyph_ids: HashSet::new(),
            retain_tables: HashSet::new(),
        }
    }

    /// Add glyphs to keep in the subset
    pub fn with_glyphs(&mut self, glyph_ids: &[u32]) -> &mut Self {
        for &id in glyph_ids {
            self.glyph_ids.insert(id);
        }
        self
    }

    /// Add characters to keep (will be mapped to their glyphs)
    pub fn with_chars(&mut self, chars: &[char]) -> Result<&mut Self> {
        for &c in chars {
            let glyph_id = self.font.char_to_glyph(c)?;
            self.glyph_ids.insert(glyph_id as u32);
        }
        Ok(self)
    }

    /// Retain specific tables (all tables are retained by default)
    pub fn retain_tables(&mut self, tables: &[[u8; 4]]) -> &mut Self {
        for &tag in tables {
            self.retain_tables.insert(tag);
        }
        self
    }

    /// Build the subset font
    pub fn build(&self) -> Result<Font> {
        if self.glyph_ids.is_empty() {
            return Err(TtfError::ParseError("No glyphs specified for subsetting".to_string()));
        }

        let mut subset_font = self.font.clone();

        // Create glyph ID mapping (old -> new)
        let mut glyph_map: HashMap<u32, u32> = HashMap::new();
        let mut sorted_glyphs: Vec<u32> = self.glyph_ids.iter().cloned().collect();
        sorted_glyphs.sort();

        for (new_id, old_id) in sorted_glyphs.iter().enumerate() {
            glyph_map.insert(*old_id, new_id as u32);
        }

        // Always include glyph 0 (.notdef)
        if !self.glyph_ids.contains(&0) {
            glyph_map.insert(0, 0);
            sorted_glyphs.insert(0, 0);
        }

        // Create subset of tables
        self.subset_glyf_table(&mut subset_font, &glyph_map)?;
        self.subset_loca_table(&mut subset_font, &glyph_map)?;
        self.subset_hmtx_table(&mut subset_font, &glyph_map)?;
        self.subset_cmap_table(&mut subset_font, &glyph_map)?;

        // Update maxp table
        self.update_maxp_table(&mut subset_font)?;

        // Remove unused tables if specified
        if !self.retain_tables.is_empty() {
            subset_font.table_records.retain(|r| self.retain_tables.contains(&r.table_tag));
        }

        Ok(subset_font)
    }

    /// Subset the glyf table
    fn subset_glyf_table(&self, font: &mut Font, glyph_map: &HashMap<u32, u32>) -> Result<()> {
        let glyf_table = self.font.glyf_table()?;
        let mut new_glyphs = Vec::new();

        for (old_id, new_id) in glyph_map {
            if let Some(glyph) = glyf_table.get_glyph(*old_id as usize) {
                new_glyphs.push((*new_id as usize, glyph.clone()));
            }
        }

        // Sort by new glyph ID
        new_glyphs.sort_by_key(|(id, _)| *id);

        // Create new glyf data
        let mut writer = FontWriter::new();

        for (_, glyph) in &new_glyphs {
            match &glyph.data {
                GlyphData::Simple(simple) => {
                    writer.write_i16(glyph.number_of_contours);
                    writer.write_i16(glyph.x_min);
                    writer.write_i16(glyph.y_min);
                    writer.write_i16(glyph.x_max);
                    writer.write_i16(glyph.y_max);

                    for &end_pt in &simple.end_pts_of_contours {
                        writer.write_u16(end_pt);
                    }

                    writer.write_u16(simple.instruction_length);
                    for &instr in &simple.instructions {
                        writer.write_u8(instr);
                    }

                    for &flag in &simple.flags {
                        writer.write_u8(flag);
                    }

                    for &x in &simple.x_coordinates {
                        writer.write_i16(x);
                    }

                    for &y in &simple.y_coordinates {
                        writer.write_i16(y);
                    }
                }
                GlyphData::Composite(composite) => {
                    writer.write_i16(glyph.number_of_contours);
                    writer.write_i16(glyph.x_min);
                    writer.write_i16(glyph.y_min);
                    writer.write_i16(glyph.x_max);
                    writer.write_i16(glyph.y_max);

                    for component in &composite.components {
                        writer.write_u16(component.flags);
                        let new_glyph_id = glyph_map.get(&(component.glyph_index as u32))
                            .copied()
                            .unwrap_or(component.glyph_index as u32);
                        writer.write_u16(new_glyph_id as u16);
                        writer.write_i16(component.arg1);
                        writer.write_i16(component.arg2);
                    }
                }
                GlyphData::Empty => {
                    writer.write_i16(0);
                    writer.write_i16(0);
                    writer.write_i16(0);
                    writer.write_i16(0);
                    writer.write_i16(0);
                }
            }
        }

        // Update font data (simplified)
        if let Some(record) = font.get_table_record(b"glyf") {
            let data = writer.into_inner();
            let offset = record.offset as usize;
            if offset + data.len() <= font.data.len() {
                font.data[offset..offset + data.len()].copy_from_slice(&data);
                if let Some(record) = font.table_records.iter_mut().find(|r| r.table_tag == *b"glyf") {
                    record.length = data.len() as u32;
                }
            }
        }

        Ok(())
    }

    /// Subset the loca table
    fn subset_loca_table(&self, font: &mut Font, glyph_map: &HashMap<u32, u32>) -> Result<()> {
        let head = self.font.head_table()?;
        let num_glyphs = glyph_map.len();

        let mut offsets = Vec::new();
        let mut current_offset = 0u32;

        for i in 0..num_glyphs {
            offsets.push(current_offset);

            // Get glyph size (simplified - in practice you'd calculate this from glyf data)
            if let Some(glyph) = self.font.glyf_table()?.get_glyph(i) {
                let size = match &glyph.data {
                    GlyphData::Simple(simple) => {
                        10 + (simple.end_pts_of_contours.len() * 2) as u32 +
                        simple.instruction_length as u32 +
                        simple.flags.len() as u32 +
                        (simple.x_coordinates.len() * 2) as u32 +
                        (simple.y_coordinates.len() * 2) as u32
                    }
                    GlyphData::Composite(composite) => {
                        10 + (composite.components.len() * 8) as u32
                    }
                    GlyphData::Empty => 10,
                };
                current_offset += size;
            }
        }

        let mut writer = FontWriter::new();
        if head.is_long_loca_format() {
            for &offset in &offsets {
                writer.write_u32(offset);
            }
        } else {
            for &offset in &offsets {
                writer.write_u16((offset / 2) as u16);
            }
        }

        // Update font data (simplified)
        if let Some(record) = font.get_table_record(b"loca") {
            let data = writer.into_inner();
            let offset = record.offset as usize;
            if offset + data.len() <= font.data.len() {
                font.data[offset..offset + data.len()].copy_from_slice(&data);
                if let Some(record) = font.table_records.iter_mut().find(|r| r.table_tag == *b"loca") {
                    record.length = data.len() as u32;
                }
            }
        }

        Ok(())
    }

    /// Subset the hmtx table
    fn subset_hmtx_table(&self, font: &mut Font, glyph_map: &HashMap<u32, u32>) -> Result<()> {
        let hmtx = self.font.hmtx_table()?;
        let _hhea = self.font.hhea_table()?;

        let mut writer = FontWriter::new();

        for i in 0..glyph_map.len() {
            let advance = hmtx.get_advance_width(i as u16);
            let lsb = hmtx.get_lsb(i as u16);

            writer.write_u16(advance);
            writer.write_i16(lsb);
        }

        // Update font data (simplified)
        if let Some(record) = font.get_table_record(b"hmtx") {
            let data = writer.into_inner();
            let offset = record.offset as usize;
            if offset + data.len() <= font.data.len() {
                font.data[offset..offset + data.len()].copy_from_slice(&data);
                if let Some(record) = font.table_records.iter_mut().find(|r| r.table_tag == *b"hmtx") {
                    record.length = data.len() as u32;
                }
            }
        }

        Ok(())
    }

    /// Subset the cmap table
    fn subset_cmap_table(&self, _font: &mut Font, glyph_map: &HashMap<u32, u32>) -> Result<()> {
        let _cmap = self.font.cmap_table()?;

        // Build reverse mapping (new -> old)
        let _reverse_map: HashMap<u32, u32> = glyph_map.iter().map(|(k, v)| (*v, *k)).collect();

        // For each character, find if its glyph is in the subset
        // This is a simplified implementation - a full implementation would
        // rebuild the entire cmap subtable structure

        // For now, just note that we need to update cmap
        // A full implementation would rewrite the Format 4 subtable
        // to only include mappings to glyphs in the subset

        Ok(())
    }

    /// Update the maxp table with new glyph count
    fn update_maxp_table(&self, font: &mut Font) -> Result<()> {
        // Update num_glyphs in maxp table
        let new_num_glyphs = self.glyph_ids.len() as u16;

        // Update font data (simplified)
        if let Some(record) = font.get_table_record(b"maxp") {
            let offset = record.offset as usize;
            // num_glyphs is at offset 4 in maxp table
            let glyph_count_offset = offset + 4;
            if glyph_count_offset + 2 <= font.data.len() {
                font.data[glyph_count_offset..glyph_count_offset + 2]
                    .copy_from_slice(&new_num_glyphs.to_be_bytes());
            }
        }

        Ok(())
    }
}

impl Font {
    /// Create a subset of this font
    pub fn subset(self) -> FontSubset {
        FontSubset::new(self)
    }
}
