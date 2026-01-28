// Example showing how to extract font metrics
use ttf_rs::Font;

fn main() {
    let font = match Font::load("test.ttf") {
        Ok(font) => font,
        Err(e) => {
            eprintln!("Error loading font: {}", e);
            eprintln!("Please provide a valid TTF file named 'test.ttf' in the project root.");
            return;
        }
    };

    println!("=== Font Metrics Analysis ===");
    println!();

    // HEAD table metrics
    if let Ok(head) = font.head_table() {
        println!("HEAD Table (Font Header):");
        println!("  Table Version: {}", head.table_version);
        println!("  Font Revision: {}", head.font_revision);
        println!("  Checksum Adjustment: {:#x}", head.checksum_adjustment);
        println!("  Magic Number: {:#x}", head.magic_number);
        println!("  Flags: {:#06x}", head.flags);
        println!("  Units per EM: {}", head.units_per_em);
        println!("  Created: {} seconds since 1904", head.created);
        println!("  Modified: {} seconds since 1904", head.modified);
        println!("  Font Bounding Box:");
        println!("    xMin: {}", head.x_min);
        println!("    yMin: {}", head.y_min);
        println!("    xMax: {}", head.x_max);
        println!("    yMax: {}", head.y_max);
        println!("  Mac Style: {:#06x}", head.mac_style);
        println!("  Lowest Rec PPEM: {}", head.lowest_rec_ppem);
        println!("  Font Direction Hint: {}", head.font_direction_hint);
        println!("  Index to Loc Format: {}", head.index_to_loc_format);
        println!("    ({} format)", if head.is_long_loca_format() { "long" } else { "short" });
        println!("  Glyph Data Format: {}", head.glyph_data_format);
        println!();
    }

    // MAXP table metrics
    if let Ok(maxp) = font.maxp_table() {
        println!("MAXP Table (Maximum Profile):");
        println!("  Version: {}", maxp.version);
        println!("  Num Glyphs: {}", maxp.num_glyphs);

        if maxp.is_version_1_0() {
            println!("  Version 1.0 Maximums:");
            if let Some(v) = maxp.max_points { println!("    Max Points: {}", v); }
            if let Some(v) = maxp.max_contours { println!("    Max Contours: {}", v); }
            if let Some(v) = maxp.max_composite_points { println!("    Max Composite Points: {}", v); }
            if let Some(v) = maxp.max_composite_contours { println!("    Max Composite Contours: {}", v); }
            if let Some(v) = maxp.max_zones { println!("    Max Zones: {}", v); }
            if let Some(v) = maxp.max_twilight_points { println!("    Max Twilight Points: {}", v); }
            if let Some(v) = maxp.max_storage { println!("    Max Storage: {}", v); }
            if let Some(v) = maxp.max_function_defs { println!("    Max Function Defs: {}", v); }
            if let Some(v) = maxp.max_instruction_defs { println!("    Max Instruction Defs: {}", v); }
            if let Some(v) = maxp.max_stack_elements { println!("    Max Stack Elements: {}", v); }
            if let Some(v) = maxp.max_size_of_instructions { println!("    Max Size of Instructions: {} bytes", v); }
            if let Some(v) = maxp.max_component_elements { println!("    Max Component Elements: {}", v); }
            if let Some(v) = maxp.max_component_depth { println!("    Max Component Depth: {}", v); }
        }
        println!();
    }

    // HHEA table metrics
    if let Ok(hhea) = font.hhea_table() {
        println!("HHEA Table (Horizontal Header):");
        println!("  Table Version: {}", hhea.table_version);
        println!("  Ascent: {}", hhea.ascent);
        println!("  Descent: {}", hhea.descent);
        println!("  Line Gap: {}", hhea.line_gap);
        println!("  Calculated Line Height: {}", hhea.get_line_height());
        println!("  Advance Width Max: {}", hhea.advance_width_max);
        println!("  Min Left Side Bearing: {}", hhea.min_left_side_bearing);
        println!("  Min Right Side Bearing: {}", hhea.min_right_side_bearing);
        println!("  X Max Extent: {}", hhea.x_max_extent);
        println!("  Caret Slope Rise: {}", hhea.caret_slope_rise);
        println!("  Caret Slope Run: {}", hhea.caret_slope_run);
        println!("  Caret Offset: {}", hhea.caret_offset);
        println!("  Metric Data Format: {}", hhea.metric_data_format);
        println!("  Number of HMetrics: {}", hhea.number_of_h_metrics);
        println!();
    }

    // OS/2 table metrics
    if let Ok(os2) = font.os2_table() {
        println!("OS/2 Table (OS/2 & Windows Metrics):");
        println!("  Version: {}", os2.version);
        println!("  X Avg Char Width: {}", os2.x_avg_char_width);
        println!("  Weight Class: {} ({})", os2.us_weight_class, os2.get_weight_string());
        println!("  Width Class: {}", os2.us_width_class);
        println!("  Type: {:#06x}", os2.fs_type);
        println!("  Y Subscript:");
        println!("    X Size: {}", os2.y_subscript_x_size);
        println!("    Y Size: {}", os2.y_subscript_y_size);
        println!("    X Offset: {}", os2.y_subscript_x_offset);
        println!("    Y Offset: {}", os2.y_subscript_y_offset);
        println!("  Y Superscript:");
        println!("    X Size: {}", os2.y_superscript_x_size);
        println!("    Y Size: {}", os2.y_superscript_y_size);
        println!("    X Offset: {}", os2.y_superscript_x_offset);
        println!("    Y Offset: {}", os2.y_superscript_y_offset);
        println!("  Strikeout:");
        println!("    Size: {}", os2.y_strikeout_size);
        println!("    Position: {}", os2.y_strikeout_position);
        println!("  Family Class: {}", os2.s_family_class);
        println!("  Panose: {:?}", os2.panose);
        println!("  Unicode Ranges:");
        println!("    Range 1: {:#010x}", os2.ul_unicode_range1);
        println!("    Range 2: {:#010x}", os2.ul_unicode_range2);
        println!("    Range 3: {:#010x}", os2.ul_unicode_range3);
        println!("    Range 4: {:#010x}", os2.ul_unicode_range4);
        println!("  Vendor ID: {}", String::from_utf8_lossy(&os2.ach_vend_id));
        println!("  Selection: {:#06x}", os2.fs_selection);
        println!("    Is Bold: {}", os2.is_bold());
        println!("    Is Italic: {}", os2.is_italic());
        println!("  First Char Index: {} ('{}')", os2.us_first_char_index,
                 if os2.us_first_char_index > 32 {
                     char::from_u32(os2.us_first_char_index as u32)
                         .map(|c| c.to_string())
                         .unwrap_or_else(|| format!("U+{:04X}", os2.us_first_char_index))
                 } else {
                     format!("U+{:04X}", os2.us_first_char_index)
                 });
        println!("  Last Char Index: {} ('{}')", os2.us_last_char_index,
                 if os2.us_last_char_index > 32 && os2.us_last_char_index < 127 {
                     char::from_u32(os2.us_last_char_index as u32)
                         .map(|c| c.to_string())
                         .unwrap_or_else(|| format!("U+{:04X}", os2.us_last_char_index))
                 } else {
                     format!("U+{:04X}", os2.us_last_char_index)
                 });
        println!("  Typo Ascender: {}", os2.s_typo_ascender);
        println!("  Typo Descender: {}", os2.s_typo_descender);
        println!("  Typo Line Gap: {}", os2.s_typo_line_gap);
        println!("  Win Ascent: {}", os2.us_win_ascent);
        println!("  Win Descent: {}", os2.us_win_descent);
        println!("  Code Page Ranges:");
        println!("    Range 1: {:#010x}", os2.ul_code_page_range1);
        println!("    Range 2: {:#010x}", os2.ul_code_page_range2);

        if os2.version >= 1 {
            println!("  X Height: {}", os2.sx_height);
            println!("  Cap Height: {}", os2.s_cap_height);
            println!("  Default Char: {}", os2.us_default_char);
            println!("  Break Char: {}", os2.us_break_char);
            println!("  Max Context: {}", os2.us_max_context);
        }
        println!();
    }

    // POST table metrics
    if let Ok(post) = font.post_table() {
        println!("POST Table (PostScript Information):");
        println!("  Format: {}", post.format);
        println!("  Italic Angle: {}", post.italic_angle);
        println!("  Underline Position: {}", post.underline_position);
        println!("  Underline Thickness: {}", post.underline_thickness);
        println!("  Fixed Pitch: {}", post.is_fixed_pitch != 0);
        println!();
    }

    // Useful calculations
    println!("=== Useful Calculations ===");

    if let (Ok(head), Ok(hhea), Ok(os2)) = (font.head_table(), font.hhea_table(), font.os2_table()) {
        let upem = head.units_per_em as f32;

        println!("Font size scaling (at {} UPEM):", upem);

        let sizes = [12.0, 16.0, 24.0, 32.0, 48.0, 72.0];
        for size in sizes {
            let scale = size / upem;
            println!("  At {}pt:", size);
            println!("    Ascender: {:.1}px", hhea.ascent as f32 * scale);
            println!("    Descender: {:.1}px", hhea.descent.abs() as f32 * scale);
            println!("    Line height: {:.1}px", hhea.get_line_height() as f32 * scale);
            println!("    Strikeout: {:.1}px", os2.y_strikeout_size as f32 * scale);
        }
    }
}
