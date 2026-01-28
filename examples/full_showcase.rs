// Full showcase of ttf-rs capabilities with real font operations
//
// This example demonstrates all major features of the library:
// - Font loading and inspection
// - Table parsing and manipulation
// - Character to glyph mapping
// - Font modification
// - Font subsetting
// - Font validation
// - Performance optimization with caching
// - Format conversion
// - Advanced glyph operations

use ttf_rs::{
    Font, FontModifier, FontSubset, ValidationReport, CachedFont,
    HeadTable, MaxpTable, CmapTable, CmapSubtable, NameTable, HheaTable,
    GlyphData, BoundingBox,
};
use std::path::Path;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║   ttf-rs: Full Capability Showcase                    ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let font_path = Path::new(&args[1]);
    println!("Loading font: {}", font_path.display());
    
    let font = Font::load(font_path)?;
    println!("✓ Font loaded successfully\n");

    // Run all capability demonstrations
    demonstrate_basic_info(&font)?;
    demonstrate_table_parsing(&font)?;
    demonstrate_character_mapping(&font)?;
    demonstrate_glyph_operations(&font)?;
    demonstrate_font_metrics(&font)?;
    demonstrate_validation(&font)?;
    demonstrate_caching(&font)?;
    
    // Modification and subsetting (creates new files)
    if args.len() > 2 && args[2] == "--with-output" {
        demonstrate_modification(font.clone())?;
        demonstrate_subsetting(&font)?;
        demonstrate_format_conversion(&font)?;
    } else {
        println!("ℹ️  Run with --with-output to test modification, subsetting, and conversion\n");
    }

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║   All Demonstrations Complete                         ║");
    println!("╚════════════════════════════════════════════════════════╝");
    
    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run --example full_showcase <font-file.ttf> [--with-output]");
    println!();
    println!("This example demonstrates all ttf-rs capabilities:");
    println!("  ✓ Font loading and inspection");
    println!("  ✓ Table parsing (20+ table types)");
    println!("  ✓ Character to glyph mapping");
    println!("  ✓ Glyph operations (bounding box, transformation)");
    println!("  ✓ Font metrics extraction");
    println!("  ✓ Font validation");
    println!("  ✓ Performance optimization with caching");
    println!("  ✓ Font modification (with --with-output)");
    println!("  ✓ Font subsetting (with --with-output)");
    println!("  ✓ Format conversion (with --with-output)");
    println!();
    println!("Provide a TTF/OTF file path to see full demonstration.");
}

fn demonstrate_basic_info(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 1. BASIC FONT INFORMATION ═══");
    
    println!("SFNT Version: 0x{:08X}", font.sfnt_version);
    println!("Number of tables: {}", font.num_tables);
    
    let tables: Vec<String> = font.table_records.iter()
        .map(|r| r.tag_to_string())
        .collect();
    println!("Tables present: {}", tables.join(", "));
    
    if let Ok(num_glyphs) = font.num_glyphs() {
        println!("Total glyphs: {}", num_glyphs);
    }
    
    if let Ok(units_per_em) = font.units_per_em() {
        println!("Units per EM: {}", units_per_em);
    }
    
    println!();
    Ok(())
}

fn demonstrate_table_parsing(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 2. TABLE PARSING ═══");
    
    // HEAD table
    if let Ok(head) = font.head_table() {
        println!("HEAD Table:");
        println!("  Font revision: {}", head.font_revision);
        println!("  Units per EM: {}", head.units_per_em);
        println!("  Bounding box: ({}, {}) to ({}, {})",
            head.x_min, head.y_min, head.x_max, head.y_max);
        println!("  Created: {}", head.created);
        println!("  Modified: {}", head.modified);
        println!("  Flags: 0x{:04X}", head.flags);
        println!("  Index to loc format: {} ({})",
            head.index_to_loc_format,
            if head.is_long_loca_format() { "long" } else { "short" });
    }
    
    // MAXP table
    if let Ok(maxp) = font.maxp_table() {
        println!("\nMAXP Table:");
        println!("  Version: {}", maxp.version);
        println!("  Num glyphs: {}", maxp.num_glyphs);
        if maxp.is_version_1_0() {
            if let Some(v) = maxp.max_points { println!("  Max points: {}", v); }
            if let Some(v) = maxp.max_contours { println!("  Max contours: {}", v); }
            if let Some(v) = maxp.max_composite_points { println!("  Max composite points: {}", v); }
            if let Some(v) = maxp.max_composite_contours { println!("  Max composite contours: {}", v); }
        }
    }
    
    // CMAP table
    if let Ok(cmap) = font.cmap_table() {
        println!("\nCMAP Table:");
        println!("  Version: {}", cmap.version);
        println!("  Number of encoding tables: {}", cmap.encoding_records.len());
        for (i, record) in cmap.encoding_records.iter().enumerate().take(5) {
            let format_str = match cmap.subtables.get(i) {
                Some(CmapSubtable::Format0(_)) => "0",
                Some(CmapSubtable::Format4(_)) => "4",
                Some(CmapSubtable::Format6(_)) => "6",
                Some(CmapSubtable::Format12(_)) => "12",
                Some(CmapSubtable::Format13(_)) => "13",
                Some(CmapSubtable::Format14(_)) => "14",
                None => "unknown",
            };
            println!("  Encoding {}: platform={}, encoding={}, format={}",
                i, record.platform_id, record.encoding_id, format_str);
        }
    }
    
    // NAME table
    if let Ok(name) = font.name_table() {
        println!("\nNAME Table:");
        println!("  Format: {}", name.format);
        println!("  Count: {}", name.count);
        
        if let Some(family) = name.get_font_name() {
            println!("  Font family: name_id={}", family.name_id);
        }
        if let Some(full_name) = name.get_full_name() {
            println!("  Full name: name_id={}", full_name.name_id);
        }
        if let Some(ps_name) = name.get_postscript_name() {
            println!("  PostScript name: name_id={}", ps_name.name_id);
        }
    }
    
    // HHEA table
    if let Ok(hhea) = font.hhea_table() {
        println!("\nHHEA Table:");
        println!("  Version: {}", hhea.table_version);
        println!("  Ascender: {}", hhea.ascent);
        println!("  Descender: {}", hhea.descent);
        println!("  Line gap: {}", hhea.line_gap);
        println!("  Line height: {}", hhea.get_line_height());
        println!("  Max advance width: {}", hhea.advance_width_max);
        println!("  Num H metrics: {}", hhea.number_of_h_metrics);
    }
    
    // POST table
    if let Ok(post) = font.post_table() {
        println!("\nPOST Table:");
        println!("  Format: {}", post.format);
        println!("  Italic angle: {}", post.italic_angle);
        println!("  Underline position: {}", post.underline_position);
        println!("  Underline thickness: {}", post.underline_thickness);
        println!("  Is fixed pitch: {}", post.is_fixed_pitch != 0);
    }
    
    // OS/2 table
    if let Ok(os2) = font.os2_table() {
        println!("\nOS/2 Table:");
        println!("  Version: {}", os2.version);
        println!("  Weight class: {} ({})", os2.us_weight_class, os2.get_weight_string());
        println!("  Width class: {}", os2.us_width_class);
        println!("  Bold: {}", os2.is_bold());
        println!("  Italic: {}", os2.is_italic());
        println!("  Embedding permissions: 0x{:04X}", os2.fs_type);
        println!("  Typo ascender: {}", os2.s_typo_ascender);
        println!("  Typo descender: {}", os2.s_typo_descender);
        println!("  Typo line gap: {}", os2.s_typo_line_gap);
    }
    
    println!();
    Ok(())
}

fn demonstrate_character_mapping(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 3. CHARACTER TO GLYPH MAPPING ═══");
    
    let test_chars = [
        'A', 'B', 'C', 'a', 'b', 'c',
        '0', '1', '2', '!', '@', '#',
        ' ', '.', ',', '?',
    ];
    
    println!("Mapping characters to glyph IDs:");
    for &c in &test_chars {
        match font.char_to_glyph(c) {
            Ok(glyph_id) => {
                println!("  '{}' (U+{:04X}) -> glyph {}", c, c as u32, glyph_id);
            }
            Err(_) => {
                println!("  '{}' (U+{:04X}) -> not found", c, c as u32);
            }
        }
    }
    
    // Test Unicode beyond BMP if cmap supports it
    let extended_chars = ['😀', '中', '日', '한'];
    println!("\nExtended Unicode characters:");
    for &c in &extended_chars {
        match font.char_to_glyph(c) {
            Ok(glyph_id) => {
                println!("  '{}' (U+{:04X}) -> glyph {}", c, c as u32, glyph_id);
            }
            Err(_) => {
                println!("  '{}' (U+{:04X}) -> not found", c, c as u32);
            }
        }
    }
    
    println!();
    Ok(())
}

fn demonstrate_glyph_operations(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 4. GLYPH OPERATIONS ═══");
    
    if let Ok(glyf) = font.glyf_table() {
        println!("Total glyphs in glyf table: {}", glyf.glyphs.len());
        
        let mut simple_count = 0;
        let mut composite_count = 0;
        let mut empty_count = 0;
        
        for glyph in &glyf.glyphs {
            match &glyph.data {
                GlyphData::Simple(_) => simple_count += 1,
                GlyphData::Composite(_) => composite_count += 1,
                GlyphData::Empty => empty_count += 1,
            }
        }
        
        println!("  Simple glyphs: {}", simple_count);
        println!("  Composite glyphs: {}", composite_count);
        println!("  Empty glyphs: {}", empty_count);
        
        // Analyze first few non-empty glyphs
        println!("\nAnalyzing first 5 non-empty glyphs:");
        let mut analyzed = 0;
        for (i, glyph) in glyf.glyphs.iter().enumerate() {
            if analyzed >= 5 {
                break;
            }
            
            match &glyph.data {
                GlyphData::Simple(simple) => {
                    println!("  Glyph {}: Simple", i);
                    println!("    Contours: {}", glyph.number_of_contours);
                    println!("    Points: {}", simple.x_coordinates.len());
                    println!("    Bounding box: ({}, {}) to ({}, {})",
                        glyph.x_min, glyph.y_min, glyph.x_max, glyph.y_max);
                    
                    if let Some(bbox) = glyph.calculate_bounding_box() {
                        println!("    Calculated bbox: width={:.2}, height={:.2}",
                            bbox.width(), bbox.height());
                    }
                    analyzed += 1;
                }
                GlyphData::Composite(composite) => {
                    println!("  Glyph {}: Composite", i);
                    println!("    Components: {}", composite.components.len());
                    println!("    Bounding box: ({}, {}) to ({}, {})",
                        glyph.x_min, glyph.y_min, glyph.x_max, glyph.y_max);
                    analyzed += 1;
                }
                GlyphData::Empty => {}
            }
        }
    }
    
    println!();
    Ok(())
}

fn demonstrate_font_metrics(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 5. FONT METRICS ═══");
    
    if let Ok(hmtx) = font.hmtx_table() {
        println!("Horizontal Metrics:");
        println!("  H metrics count: {}", hmtx.h_metrics.len());
        println!("  Left side bearings count: {}", hmtx.left_side_bearings.len());
        
        println!("\nFirst 10 glyph metrics:");
        for (i, metric) in hmtx.h_metrics.iter().enumerate().take(10) {
            println!("  Glyph {}: advance={}, lsb={}",
                i, metric.advance_width, metric.lsb);
        }
    }
    
    println!();
    Ok(())
}

fn demonstrate_validation(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 6. FONT VALIDATION ═══");
    
    println!("Running comprehensive validation...");
    let report = font.validate()?;
    
    println!("Validation result: {}", if report.is_valid { "✓ VALID" } else { "✗ INVALID" });
    println!("Errors: {}", report.errors.len());
    println!("Warnings: {}", report.warnings.len());
    
    if !report.errors.is_empty() {
        println!("\nErrors found:");
        for (i, error) in report.errors.iter().enumerate().take(10) {
            println!("  {}. {:?}", i + 1, error);
        }
        if report.errors.len() > 10 {
            println!("  ... and {} more errors", report.errors.len() - 10);
        }
    }
    
    if !report.warnings.is_empty() {
        println!("\nWarnings:");
        for (i, warning) in report.warnings.iter().enumerate().take(10) {
            println!("  {}. {:?}", i + 1, warning);
        }
        if report.warnings.len() > 10 {
            println!("  ... and {} more warnings", report.warnings.len() - 10);
        }
    }
    
    println!();
    Ok(())
}

fn demonstrate_caching(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 7. PERFORMANCE OPTIMIZATION (CACHING) ═══");
    
    use std::time::Instant;
    
    // Without caching
    let start = Instant::now();
    for _ in 0..100 {
        let _ = font.head_table();
        let _ = font.maxp_table();
        let _ = font.cmap_table();
    }
    let without_cache = start.elapsed();
    
    // With caching
    let cached = font.clone().with_cache();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = cached.head_table_cached();
        let _ = cached.maxp_table_cached();
        let _ = cached.cmap_table_cached();
    }
    let with_cache = start.elapsed();
    
    println!("Performance comparison (100 iterations):");
    println!("  Without caching: {:?}", without_cache);
    println!("  With caching: {:?}", with_cache);
    println!("  Speedup: {:.2}x", without_cache.as_secs_f64() / with_cache.as_secs_f64());
    
    println!();
    Ok(())
}

fn demonstrate_modification(font: Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 8. FONT MODIFICATION ═══");
    
    let mut modifier = FontModifier::new(font);
    
    println!("Modifying font properties...");
    modifier.set_font_name("Modified Demo Font")?;
    modifier.set_version(2, 0)?;
    modifier.set_font_metrics(1000, 800, -250, 0)?;
    
    println!("  ✓ Set font name");
    println!("  ✓ Set version to 2.0");
    println!("  ✓ Set font metrics");
    
    let output_path = "output/modified_font.ttf";
    std::fs::create_dir_all("output")?;
    let modified_font = modifier.commit()?;
    modified_font.save(output_path)?;
    
    println!("  ✓ Saved to {}", output_path);
    println!();
    
    Ok(())
}

fn demonstrate_subsetting(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 9. FONT SUBSETTING ═══");
    
    let chars_to_keep = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ' ', '.', ',', '!', '?',
    ];
    
    println!("Creating subset with {} characters...", chars_to_keep.len());
    
    let mut subset = font.clone().subset();
    subset.with_chars(&chars_to_keep)?;
    
    let subset_font = subset.build()?;
    
    let output_path = "output/subset_font.ttf";
    subset_font.save(output_path)?;
    
    println!("  ✓ Subset created");
    println!("  ✓ Saved to {}", output_path);
    
    // Compare sizes
    let original_size = font.data.len();
    let subset_size = subset_font.data.len();
    let reduction = 100.0 * (1.0 - subset_size as f64 / original_size as f64);
    
    println!("  Original size: {} bytes", original_size);
    println!("  Subset size: {} bytes", subset_size);
    println!("  Size reduction: {:.1}%", reduction);
    
    println!();
    Ok(())
}

fn demonstrate_format_conversion(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ 10. FORMAT CONVERSION ═══");
    
    println!("Converting to WOFF format...");
    match font.to_woff() {
        Ok(woff_data) => {
            let output_path = "output/converted.woff";
            std::fs::write(output_path, &woff_data)?;
            println!("  ✓ Converted to WOFF");
            println!("  ✓ Saved to {}", output_path);
            println!("  Size: {} bytes", woff_data.len());
        }
        Err(e) => {
            println!("  ⚠ WOFF conversion: {}", e);
        }
    }
    
    println!("\nConverting to WOFF2 format...");
    match font.to_woff2() {
        Ok(woff2_data) => {
            let output_path = "output/converted.woff2";
            std::fs::write(output_path, &woff2_data)?;
            println!("  ✓ Converted to WOFF2");
            println!("  ✓ Saved to {}", output_path);
            println!("  Size: {} bytes", woff2_data.len());
        }
        Err(e) => {
            println!("  ⚠ WOFF2 conversion: {}", e);
        }
    }
    
    println!();
    Ok(())
}
