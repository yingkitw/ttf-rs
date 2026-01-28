use crate::error::{Result, TtfError};
use crate::font::Font;
use crate::stream::FontWriter;
use crate::tables::glyf::GlyphData;
use crate::tables::cmap::{CmapSubtable, Format4};
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
    fn subset_cmap_table(&self, font: &mut Font, glyph_map: &HashMap<u32, u32>) -> Result<()> {
        let cmap = self.font.cmap_table()?;

        // Build character to new glyph mapping for all characters in the original cmap
        let mut char_to_new_glyph: Vec<(u32, u32)> = Vec::new();

        // Iterate through all possible characters and find mappings
        for subtable in &cmap.subtables {
            match subtable {
                CmapSubtable::Format4(format4) => {
                    // For Format 4, collect all character mappings
                    for seg in 0..format4.seg_count() as usize {
                        let start_code = format4.start_codes[seg];
                        let end_code = format4.end_codes[seg];
                        let id_delta = format4.id_deltas[seg];

                        for char_code in start_code..=end_code {
                            let old_glyph = if format4.id_range_offsets[seg] == 0 {
                                ((char_code as i32 + id_delta as i32) as u16)
                            } else {
                                // For complex cases, use the lookup
                                if let Some(g) = format4.get_glyph(char_code) {
                                    g
                                } else {
                                    continue;
                                }
                            };

                            // Check if this glyph is in our subset
                            if let Some(&new_glyph) = glyph_map.get(&(old_glyph as u32)) {
                                char_to_new_glyph.push((char_code as u32, new_glyph));
                            }
                        }
                    }
                }
                CmapSubtable::Format6(format6) => {
                    for (i, &old_glyph) in format6.glyph_id_array.iter().enumerate() {
                        let char_code = format6.first_code as u32 + i as u32;
                        if let Some(&new_glyph) = glyph_map.get(&(old_glyph as u32)) {
                            char_to_new_glyph.push((char_code, new_glyph));
                        }
                    }
                }
                CmapSubtable::Format12(format12) => {
                    for group in &format12.groups {
                        for char_code in group.start_char_code..=group.end_char_code {
                            let old_glyph = group.start_glyph_code + (char_code - group.start_char_code);
                            if let Some(&new_glyph) = glyph_map.get(&old_glyph) {
                                char_to_new_glyph.push((char_code, new_glyph));
                            }
                        }
                    }
                }
                CmapSubtable::Format13(format13) => {
                    for group in &format13.groups {
                        for char_code in group.start_char_code..=group.end_char_code {
                            if let Some(&new_glyph) = glyph_map.get(&group.glyph_code) {
                                char_to_new_glyph.push((char_code, new_glyph));
                            }
                        }
                    }
                }
                _ => {
                    // Format 0 and Format 14 are less common, skip for now
                }
            }
        }

        // Remove duplicates and sort
        char_to_new_glyph.sort();
        char_to_new_glyph.dedup();

        // Build a new Format 4 subtable with the subset
        if !char_to_new_glyph.is_empty() {
            let new_format4 = self.build_format4_subset(&char_to_new_glyph)?;
            self.write_cmap_subset(font, &new_format4)?;
        }

        Ok(())
    }

    /// Build a Format 4 subtable from character mappings
    fn build_format4_subset(&self, mappings: &[(u32, u32)]) -> Result<Format4> {
        if mappings.is_empty() {
            return Err(TtfError::ParseError("No mappings for subset".to_string()));
        }

        // Build segments from continuous character ranges with sequential glyphs
        let mut segments: Vec<(u16, u16, i16)> = Vec::new(); // (start, end, id_delta)

        let mut current_start = mappings[0].0 as u16;
        let mut current_end = current_start;
        let mut current_glyph_delta = (mappings[0].1 as i32 - mappings[0].0 as i32) as i16;

        for &(char_code, new_glyph) in mappings.iter().skip(1) {
            let char_code = char_code as u16;
            let expected_glyph = (char_code as i32 + current_glyph_delta as i32) as u32;

            if char_code == current_end + 1 && new_glyph == expected_glyph {
                // Continue current segment
                current_end = char_code;
            } else {
                // Start new segment
                segments.push((current_start, current_end, current_glyph_delta));
                current_start = char_code;
                current_end = char_code;
                current_glyph_delta = (new_glyph as i32 - char_code as i32) as i16;
            }
        }

        // Add last segment
        segments.push((current_start, current_end, current_glyph_delta));

        // Add the final sentinel segment (0xFFFF, 0xFFFF, 1)
        segments.push((0xFFFF, 0xFFFF, 1));

        // Extract arrays from segments
        let seg_count = segments.len() as u16;
        let seg_count_x2 = seg_count * 2;
        let end_codes: Vec<u16> = segments.iter().map(|(_, end, _)| *end).collect();
        let start_codes: Vec<u16> = segments.iter().map(|(start, _, _)| *start).collect();
        let id_deltas: Vec<i16> = segments.iter().map(|(_, _, delta)| *delta).collect();

        // Calculate search range, entry selector, range_shift
        let mut search_range = 1u16;
        let mut entry_selector = 0u16;
        while search_range * 2 <= seg_count_x2 {
            search_range *= 2;
            entry_selector += 1;
        }
        let range_shift = seg_count_x2 - search_range;

        // For this simplified implementation, id_range_offsets are all 0
        let id_range_offsets = vec![0u16; seg_count as usize];
        let glyph_id_array = Vec::new(); // Empty since we use id_delta

        let length = 2 + 2 + 2 + 2 + 2 + 2 + 2 // header
            + (seg_count as u16 * 2) * 4 // end_codes, start_codes, id_deltas, id_range_offsets
            + 2 // reserved
            + (glyph_id_array.len() as u16 * 2);

        Ok(Format4 {
            format: 4,
            length,
            language: 0,
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

    /// Write the subset cmap back to the font
    fn write_cmap_subset(&self, font: &mut Font, format4: &Format4) -> Result<()> {
        let mut writer = FontWriter::new();

        // cmap header
        writer.write_u16(0); // version
        writer.write_u16(1); // num_tables (just one subtable)

        // Encoding record (Unicode BMP)
        writer.write_u16(3); // platform_id (Windows)
        writer.write_u16(1); // encoding_id (Unicode BMP)
        writer.write_u32(12); // offset (after header + encoding record)

        // Format 4 subtable
        writer.write_u16(format4.format);
        writer.write_u16(format4.length);
        writer.write_u16(format4.language);
        writer.write_u16(format4.seg_count_x2);
        writer.write_u16(format4.search_range);
        writer.write_u16(format4.entry_selector);
        writer.write_u16(format4.range_shift);

        for &end_code in &format4.end_codes {
            writer.write_u16(end_code);
        }

        writer.write_u16(0); // reservedPad

        for &start_code in &format4.start_codes {
            writer.write_u16(start_code);
        }

        for &id_delta in &format4.id_deltas {
            writer.write_i16(id_delta);
        }

        for &id_range_offset in &format4.id_range_offsets {
            writer.write_u16(id_range_offset);
        }

        for &glyph_id in &format4.glyph_id_array {
            writer.write_u16(glyph_id);
        }

        // Update font data
        if let Some(record) = font.get_table_record(b"cmap") {
            let data = writer.into_inner();
            let offset = record.offset as usize;
            if offset + data.len() <= font.data.len() {
                font.data[offset..offset + data.len()].copy_from_slice(&data);
                if let Some(record) = font.table_records.iter_mut().find(|r| r.table_tag == *b"cmap") {
                    record.length = data.len() as u32;
                }
            }
        }

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
