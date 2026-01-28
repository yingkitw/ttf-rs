use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <font.ttf>", args[0]);
        std::process::exit(1);
    }
    
    let font_path = &args[1];
    let font = Font::load(font_path)?;
    
    println!("Character to Glyph Mapping Example");
    println!("===================================\n");
    
    println!("Font: {}\n", font_path);
    
    let test_chars = vec!['A', 'B', 'C', 'a', 'b', 'c', '0', '1', '2', ' ', '!', '?'];
    
    println!("Character mappings:");
    let hmtx = font.hmtx_table().ok();
    
    for ch in test_chars {
        match font.char_to_glyph(ch) {
            Ok(glyph_id) => {
                println!("  '{}' (U+{:04X}) -> Glyph ID {}", ch, ch as u32, glyph_id);
                
                if let Some(ref hmtx_table) = hmtx {
                    let advance = hmtx_table.get_advance_width(glyph_id as u16);
                    println!("      Advance width: {}", advance);
                }
            }
            Err(e) => {
                println!("  '{}' (U+{:04X}) -> Error: {}", ch, ch as u32, e);
            }
        }
    }
    
    println!("\nUnicode range test:");
    let unicode_samples = vec![
        ('€', "Euro sign"),
        ('©', "Copyright"),
        ('™', "Trademark"),
        ('→', "Right arrow"),
        ('♥', "Heart"),
    ];
    
    for (ch, desc) in unicode_samples {
        match font.char_to_glyph(ch) {
            Ok(glyph_id) => {
                println!("  {} '{}' (U+{:04X}) -> Glyph ID {}", desc, ch, ch as u32, glyph_id);
            }
            Err(_) => {
                println!("  {} '{}' (U+{:04X}) -> Not available", desc, ch, ch as u32);
            }
        }
    }
    
    Ok(())
}
