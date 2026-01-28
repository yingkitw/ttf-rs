// Example showing how to extract detailed glyph information
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

    println!("=== Glyph Information Example ===");
    println!();

    // Get glyph count
    let num_glyphs = match font.num_glyphs() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error getting glyph count: {}", e);
            return;
        }
    };

    println!("Total glyphs in font: {}", num_glyphs);
    println!();

    // Get horizontal metrics
    if let Ok(hmtx) = font.hmtx_table() {
        println!("=== Horizontal Metrics (first 20 glyphs) ===");
        for i in 0..20.min(num_glyphs) {
            let advance = hmtx.get_advance_width(i);
            let lsb = hmtx.get_lsb(i);
            println!("Glyph {}: advance={}, lsb={}", i, advance, lsb);
        }
        println!();
    }

    // Get glyph data
    if let Ok(glyf) = font.glyf_table() {
        println!("=== Glyph Data Analysis ===");

        let mut simple_count = 0;
        let mut composite_count = 0;
        let mut empty_count = 0;

        // Analyze all glyphs
        for i in 0..num_glyphs {
            if let Some(glyph) = glyf.get_glyph(i as usize) {
                if glyph.is_empty() {
                    empty_count += 1;
                } else if glyph.is_simple() {
                    simple_count += 1;
                } else if glyph.is_composite() {
                    composite_count += 1;
                }
            }
        }

        println!("Empty glyphs: {}", empty_count);
        println!("Simple glyphs: {}", simple_count);
        println!("Composite glyphs: {}", composite_count);
        println!();

        // Show detailed info for a few simple glyphs
        println!("=== Simple Glyph Details ===");
        let mut shown = 0;
        for i in 0..num_glyphs {
            if let Some(glyph) = glyf.get_glyph(i as usize) {
                if let ttf_rs::GlyphData::Simple(data) = &glyph.data {
                    if shown < 5 {
                        println!("Glyph {} (simple):", i);
                        println!("  Contours: {}", data.end_pts_of_contours.len());
                        println!("  Points: {}",
                                 data.x_coordinates.len().min(data.y_coordinates.len()));
                        println!("  Instructions: {}", data.instruction_length);
                        println!();
                        shown += 1;
                    }
                }
            }
        }

        // Show detailed info for a composite glyph
        println!("=== Composite Glyph Details ===");
        for i in 0..num_glyphs {
            if let Some(glyph) = glyf.get_glyph(i as usize) {
                if let ttf_rs::GlyphData::Composite(data) = &glyph.data {
                    println!("Glyph {} (composite):", i);
                    println!("  Components: {}", data.components.len());
                    for (idx, comp) in data.components.iter().take(3).enumerate() {
                        println!("    Component {}:", idx);
                        println!("      Glyph index: {}", comp.glyph_index);
                        println!("      Flags: {:#06x}", comp.flags);
                        println!("      Transform: xx={}, xy={}, yx={}, yy={}",
                                 comp.transform.xx, comp.transform.xy,
                                 comp.transform.yx, comp.transform.yy);
                    }
                    break;
                }
            }
        }
    }

    println!();

    // Character mapping examples
    if let Ok(cmap) = font.cmap_table() {
        println!("=== Character Mapping Examples ===");

        let test_chars = ['A', 'B', 'C', 'a', 'b', 'c', '0', '1', '2',
                          '@', '#', '$', '%', ' ', '\n', '\t'];

        println!("Character mappings:");
        for c in test_chars {
            if let Some(glyph) = cmap.map_char(c) {
                let char_repr = if c.is_control() {
                    format!("0x{:04x}", c as u32)
                } else if c == ' ' {
                    String::from("'space'")
                } else {
                    format!("'{}'", c)
                };
                println!("  {:10} -> glyph {}", char_repr, glyph);
            }
        }
    }
}
