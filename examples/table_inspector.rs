use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <font.ttf>", args[0]);
        std::process::exit(1);
    }
    
    let font_path = &args[1];
    let font = Font::load(font_path)?;
    
    println!("Font Table Inspector");
    println!("====================\n");
    
    println!("Font: {}\n", font_path);
    
    println!("SFNT Header:");
    println!("  Version: {:#010x}", font.sfnt_version);
    println!("  Number of tables: {}", font.num_tables);
    println!("  Search range: {}", font.search_range);
    println!("  Entry selector: {}", font.entry_selector);
    println!("  Range shift: {}\n", font.range_shift);
    
    println!("Table Directory:");
    println!("{:<8} {:<12} {:<12} {:<12} {:<12}", "Tag", "Checksum", "Offset", "Length", "Status");
    println!("{}", "-".repeat(60));
    
    for record in &font.table_records {
        let tag = record.tag_to_string();
        let status = match tag.as_str() {
            "head" | "maxp" | "cmap" | "name" | "hhea" | "hmtx" | "glyf" | "loca" => "Essential",
            "post" | "OS/2" => "Important",
            _ => "Optional",
        };
        
        println!("{:<8} {:#010x}   {:#010x}   {:<12} {:<12}", 
                 tag, 
                 record.checksum, 
                 record.offset, 
                 record.length,
                 status);
    }
    
    println!("\nDetailed Table Information:");
    println!("===========================\n");
    
    if let Ok(head) = font.head_table() {
        println!("head - Font Header:");
        println!("  Font revision: {:.2}", head.font_revision);
        println!("  Units per EM: {}", head.units_per_em);
        println!("  Created: {} (Mac timestamp)", head.created);
        println!("  Modified: {} (Mac timestamp)", head.modified);
        println!("  Bounding box: ({}, {}) to ({}, {})", 
                 head.x_min, head.y_min, head.x_max, head.y_max);
        println!("  Mac style: {:#04x}", head.mac_style);
        println!("  Index to loc format: {}", head.index_to_loc_format);
        println!();
    }
    
    if let Ok(maxp) = font.maxp_table() {
        println!("maxp - Maximum Profile:");
        println!("  Version: {:.1}", maxp.version);
        println!("  Number of glyphs: {}", maxp.num_glyphs);
        println!();
    }
    
    if let Ok(hhea) = font.hhea_table() {
        println!("hhea - Horizontal Header:");
        println!("  Ascent: {}", hhea.ascent);
        println!("  Descent: {}", hhea.descent);
        println!("  Line gap: {}", hhea.line_gap);
        println!("  Advance width max: {}", hhea.advance_width_max);
        println!("  Number of h metrics: {}", hhea.number_of_h_metrics);
        println!();
    }
    
    if let Ok(post) = font.post_table() {
        println!("post - PostScript Information:");
        println!("  Format: {:.1}", post.format);
        println!("  Italic angle: {:.2}°", post.italic_angle);
        println!("  Underline position: {}", post.underline_position);
        println!("  Underline thickness: {}", post.underline_thickness);
        println!("  Is fixed pitch: {}", post.is_fixed_pitch != 0);
        println!();
    }
    
    if let Ok(os2) = font.os2_table() {
        println!("OS/2 - OS/2 and Windows Metrics:");
        println!("  Version: {}", os2.version);
        println!("  Weight class: {}", os2.us_weight_class);
        println!("  Width class: {}", os2.us_width_class);
        println!("  Embedding type: 0x{:04x}", os2.fs_type);
        println!("  Typo ascender: {}", os2.s_typo_ascender);
        println!("  Typo descender: {}", os2.s_typo_descender);
        println!("  Typo line gap: {}", os2.s_typo_line_gap);
        println!();
    }
    
    if let Ok(name_table) = font.name_table() {
        println!("name - Naming Table:");
        println!("  Number of name records: {}", name_table.name_records.len());
        
        if let Ok(family) = font.family_name() {
            println!("  Family name: {}", family);
        }
        if let Ok(full_name) = font.font_name() {
            println!("  Full name: {}", full_name);
        }
        println!();
    }
    
    Ok(())
}
