use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <font.ttf>", args[0]);
        std::process::exit(1);
    }
    
    let font_path = &args[1];
    let font = Font::load(font_path)?;
    
    println!("Glyph Metrics Example");
    println!("=====================\n");
    
    println!("Font: {}\n", font_path);
    
    let num_glyphs = font.num_glyphs()?;
    let units_per_em = font.units_per_em()?;
    
    println!("Font metrics:");
    println!("  Units per EM: {}", units_per_em);
    println!("  Total glyphs: {}\n", num_glyphs);
    
    let test_chars = vec![
        ('A', "Capital A"),
        ('a', "Lowercase a"),
        ('W', "Capital W"),
        ('i', "Lowercase i"),
        ('M', "Capital M"),
        ('l', "Lowercase l"),
        (' ', "Space"),
    ];
    
    println!("Character metrics:");
    println!("{:<15} {:<8} {:<12} {:<12}", "Character", "Glyph ID", "Advance", "LSB");
    println!("{}", "-".repeat(50));
    
    if let Ok(hmtx) = font.hmtx_table() {
        for (ch, desc) in test_chars {
            if let Ok(glyph_id) = font.char_to_glyph(ch) {
                let advance = hmtx.get_advance_width(glyph_id as u16);
                let lsb = hmtx.get_lsb(glyph_id as u16);
                
                println!("{:<15} {:<8} {:<12} {:<12}", 
                         format!("{} ('{}')", desc, ch),
                         glyph_id,
                         advance,
                         lsb);
            }
        }
    }
    
    println!("\nGlyph ID metrics (first 10 glyphs):");
    println!("{:<10} {:<12} {:<12}", "Glyph ID", "Advance", "LSB");
    println!("{}", "-".repeat(35));
    
    if let Ok(hmtx) = font.hmtx_table() {
        for glyph_id in 0..10.min(num_glyphs) {
            let advance = hmtx.get_advance_width(glyph_id);
            let lsb = hmtx.get_lsb(glyph_id);
            
            println!("{:<10} {:<12} {:<12}", glyph_id, advance, lsb);
        }
    }
    
    println!("\nStatistics:");
    
    if let Ok(hmtx) = font.hmtx_table() {
        let mut total_advance = 0u32;
        let mut min_advance = u16::MAX;
        let mut max_advance = 0u16;
        
        for glyph_id in 0..num_glyphs {
            let advance = hmtx.get_advance_width(glyph_id);
            total_advance += advance as u32;
            min_advance = min_advance.min(advance);
            max_advance = max_advance.max(advance);
        }
        
        let avg_advance = total_advance / num_glyphs as u32;
        
        println!("  Average advance width: {}", avg_advance);
        println!("  Minimum advance width: {}", min_advance);
        println!("  Maximum advance width: {}", max_advance);
    }
    
    Ok(())
}
