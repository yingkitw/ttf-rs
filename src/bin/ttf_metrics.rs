use ttf_rs::{Font, Result};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ttf-metrics <font-file>");
        eprintln!();
        eprintln!("Display comprehensive metrics for a TTF/OTF font file");
        std::process::exit(1);
    }

    let font_path = Path::new(&args[1]);
    let font = Font::load(font_path)?;

    println!("Font Metrics Report");
    println!("===================");
    println!();

    // File information
    println!("File Information:");
    println!("  Path: {}", args[1]);
    if let Ok(metadata) = std::fs::metadata(font_path) {
        println!("  Size: {} bytes", metadata.len());
    }
    println!();

    // SFNT header
    println!("SFNT Header:");
    println!("  SFNT Version: 0x{:08X}", font.sfnt_version);
    println!("  Number of Tables: {}", font.num_tables);
    println!("  Tables: {}", font.table_records.iter()
        .map(|r| String::from_utf8_lossy(&r.table_tag).to_string())
        .collect::<Vec<_>>()
        .join(", "));
    println!();

    // HEAD table
    if let Ok(head) = font.head_table() {
        println!("HEAD Table (Font Header):");
        println!("  Font Version: {}", head.font_revision);
        println!("  Units Per Em: {}", head.units_per_em);
        println!("  Created: {}", head.created);
        println!("  Modified: {}", head.modified);
        println!("  Bounding Box: xMin={}, yMin={}, xMax={}, yMax={}",
            head.x_min, head.y_min, head.x_max, head.y_max);
        println!("  Index To Loc Format: {} ({})",
            head.index_to_loc_format,
            if head.is_long_loca_format() { "Long" } else { "Short" });
        println!();
    }

    // MAXP table
    if let Ok(maxp) = font.maxp_table() {
        println!("MAXP Table (Maximum Profile):");
        println!("  Version: {}", maxp.version);
        println!("  Num Glyphs: {}", maxp.num_glyphs);
        if maxp.is_version_1_0() {
            if let Some(v) = maxp.max_points { println!("  Max Points: {}", v); }
            if let Some(v) = maxp.max_contours { println!("  Max Contours: {}", v); }
            if let Some(v) = maxp.max_composite_points { println!("  Max Composite Points: {}", v); }
            if let Some(v) = maxp.max_composite_contours { println!("  Max Composite Contours: {}", v); }
            if let Some(v) = maxp.max_zones { println!("  Max Zones: {}", v); }
            if let Some(v) = maxp.max_twilight_points { println!("  Max Twilight Points: {}", v); }
            if let Some(v) = maxp.max_storage { println!("  Max Storage: {}", v); }
            if let Some(v) = maxp.max_function_defs { println!("  Max FunctionDefs: {}", v); }
            if let Some(v) = maxp.max_instruction_defs { println!("  Max InstructionDefs: {}", v); }
            if let Some(v) = maxp.max_stack_elements { println!("  Max Stack Elements: {}", v); }
            if let Some(v) = maxp.max_size_of_instructions { println!("  Max SizeOfInstructions: {}", v); }
            if let Some(v) = maxp.max_component_elements { println!("  Max ComponentElements: {}", v); }
            if let Some(v) = maxp.max_component_depth { println!("  Max ComponentDepth: {}", v); }
        }
        println!();
    }

    // HHEA table
    if let Ok(hhea) = font.hhea_table() {
        println!("HHEA Table (Horizontal Header):");
        println!("  Ascender: {}", hhea.ascent);
        println!("  Descender: {}", hhea.descent);
        println!("  Line Gap: {}", hhea.line_gap);
        println!("  Advance Width Max: {}", hhea.advance_width_max);
        println!("  Min Left Side Bearing: {}", hhea.min_left_side_bearing);
        println!("  Min Right Side Bearing: {}", hhea.min_right_side_bearing);
        println!("  X Max Extent: {}", hhea.x_max_extent);
        println!("  Caret Slope Rise: {}", hhea.caret_slope_rise);
        println!("  Caret Slope Run: {}", hhea.caret_slope_run);
        println!("  Caret Offset: {}", hhea.caret_offset);
        println!("  Number of HMetrics: {}", hhea.number_of_h_metrics);
        println!();
    }

    // OS/2 table
    if let Ok(os2) = font.os2_table() {
        println!("OS/2 Table (OS/2 & Windows Metrics):");
        println!("  Version: {}", os2.version);
        println!("  Weight Class: {}", os2.us_weight_class);
        println!("  Width Class: {}", os2.us_width_class);
        println!("  Weight: {}", os2.get_weight_string());
        println!("  y Ascender: {}", os2.s_typo_ascender);
        println!("  y Descender: {}", os2.s_typo_descender);
        println!("  y Line Gap: {}", os2.s_typo_line_gap);
        println!("  Win Ascent: {}", os2.us_win_ascent);
        println!("  Win Descent: {}", os2.us_win_descent);
        println!("  X Height: {}", os2.sx_height);
        println!("  Cap Height: {}", os2.s_cap_height);
        println!("  Break Character: {}", os2.us_break_char);
        println!("  Default Character: {}", os2.us_default_char);
        println!();
    }

    // POST table
    if let Ok(post) = font.post_table() {
        println!("POST Table (PostScript):");
        println!("  Format: {}", post.format);
        println!("  Italic Angle: {}", post.italic_angle);
        println!("  Underline Position: {}", post.underline_position);
        println!("  Underline Thickness: {}", post.underline_thickness);
        println!("  Fixed Pitch: {}", post.is_fixed_pitch);
        println!();
    }

    // NAME table
    if let Ok(name) = font.name_table() {
        println!("NAME Table (Naming):");
        if let Some(family) = name.get_font_name() {
            println!("  Font Family: name_id={}, platform={}",
                family.name_id, family.platform_id);
        }
        if let Some(full) = name.get_full_name() {
            println!("  Full Name: name_id={}, platform={}",
                full.name_id, full.platform_id);
        }
        if let Some(postscript) = name.get_postscript_name() {
            println!("  PostScript Name: name_id={}, platform={}",
                postscript.name_id, postscript.platform_id);
        }
        println!("  Total Name Records: {}", name.count);
        println!();
    }

    // GLYF table summary
    if let Ok(glyf) = font.glyf_table() {
        println!("GLYF Table (Glyph Data):");
        let total_glyphs = glyf.glyphs.len();
        println!("  Total Glyphs: {}", total_glyphs);
        let mut simple_count = 0;
        let mut composite_count = 0;
        let mut empty_count = 0;
        for glyph in &glyf.glyphs {
            match glyph.data {
                ttf_rs::GlyphData::Simple(_) => simple_count += 1,
                ttf_rs::GlyphData::Composite(_) => composite_count += 1,
                ttf_rs::GlyphData::Empty => empty_count += 1,
            }
        }
        println!("  Simple Glyphs: {}", simple_count);
        println!("  Composite Glyphs: {}", composite_count);
        println!("  Empty Glyphs: {}", empty_count);
        println!();
    }

    // CMAP table
    if let Ok(cmap) = font.cmap_table() {
        println!("CMAP Table (Character Mapping):");
        println!("  Version: {}", cmap.version);
        println!("  Encoding Records:");
        for (i, record) in cmap.encoding_records.iter().enumerate() {
            println!("    {}. Platform={}, Encoding={}, Offset={}",
                i + 1, record.platform_id, record.encoding_id, record.offset);
        }
        if let Some(subtable) = cmap.get_best_subtable() {
            match subtable {
                ttf_rs::CmapSubtable::Format4(_) => {
                    println!("  Best Subtable: Format 4 (Unicode BMP)");
                }
                ttf_rs::CmapSubtable::Format12(_) => {
                    println!("  Best Subtable: Format 12 (Unicode Full Repertoire)");
                }
                _ => {
                    println!("  Best Subtable: Other format");
                }
            }
        }
        println!();
    }

    Ok(())
}
