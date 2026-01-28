// Basic example of using ttf-rs to read font information
use ttf_rs::Font;

fn main() {
    // Load a font file
    let font = match Font::load("test.ttf") {
        Ok(font) => font,
        Err(e) => {
            eprintln!("Error loading font: {}", e);
            eprintln!("Please provide a valid TTF file named 'test.ttf' in the project root.");
            return;
        }
    };

    // Print basic font information
    println!("=== TTF Font Information ===");
    println!();

    // List all tables
    println!("Tables in font:");
    for table in font.list_tables() {
        println!("  - {}", table);
    }
    println!();

    // Get header information
    if let Ok(head) = font.head_table() {
        println!("Font Header:");
        println!("  Version: {}", head.table_version);
        println!("  Font Revision: {}", head.font_revision);
        println!("  Units per EM: {}", head.units_per_em);
        println!("  Created: {}", head.created);
        println!("  Modified: {}", head.modified);
        println!("  Bounding Box: ({}, {}) to ({}, {})",
                 head.x_min, head.y_min, head.x_max, head.y_max);
        println!();
    }

    // Get maximum profile
    if let Ok(maxp) = font.maxp_table() {
        println!("Maximum Profile:");
        println!("  Version: {}", maxp.version);
        println!("  Number of glyphs: {}", maxp.num_glyphs);
        println!();
    }

    // Get character mapping
    if let Ok(cmap) = font.cmap_table() {
        println!("Character Mapping:");
        if let Some(subtable) = cmap.get_best_subtable() {
            println!("  Best subtable: {:?}", subtable);
        }

        // Map some characters to glyphs
        for c in ['A', 'B', 'C', 'a', 'b', 'c', '0', '1', '2'] {
            if let Some(glyph) = cmap.map_char(c) {
                println!("  '{}' -> glyph index: {}", c, glyph);
            }
        }
        println!();
    }

    // Get horizontal header
    if let Ok(hhea) = font.hhea_table() {
        println!("Horizontal Header:");
        println!("  Ascender: {}", hhea.ascent);
        println!("  Descender: {}", hhea.descent);
        println!("  Line Gap: {}", hhea.line_gap);
        println!("  Line Height: {}", hhea.get_line_height());
        println!("  Max Advance Width: {}", hhea.advance_width_max);
        println!();
    }

    // Get OS/2 table
    if let Ok(os2) = font.os2_table() {
        println!("OS/2 Table:");
        println!("  Version: {}", os2.version);
        println!("  Weight Class: {} ({})", os2.us_weight_class, os2.get_weight_string());
        println!("  Is Bold: {}", os2.is_bold());
        println!("  Is Italic: {}", os2.is_italic());
        println!("  Typo Ascender: {}", os2.s_typo_ascender);
        println!("  Typo Descender: {}", os2.s_typo_descender);
        println!("  Win Ascent: {}", os2.us_win_ascent);
        println!("  Win Descent: {}", os2.us_win_descent);
        println!();
    }

    // Test character to glyph mapping
    println!("Character to Glyph Mapping:");
    for c in "Hello, World!".chars() {
        match font.char_to_glyph(c) {
            Ok(glyph) => println!("  '{}' -> glyph {}", c, glyph),
            Err(e) => eprintln!("  '{}' -> error: {}", c, e),
        }
    }
}
