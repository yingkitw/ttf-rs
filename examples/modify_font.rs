use ttf_rs::Font;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <font.ttf>", args[0]);
        eprintln!("This example demonstrates font modification capabilities");
        std::process::exit(1);
    }

    let font_path = &args[1];
    
    println!("Loading font: {}", font_path);
    let font = Font::load(font_path)?;
    
    println!("\n=== Original Font Information ===");
    
    if let Ok(name_table) = font.name_table() {
        if let Some(record) = name_table.get_font_name() {
            println!("Font Family: Platform {}, Encoding {}, Language {:#x}", 
                     record.platform_id, record.encoding_id, record.language_id);
        }
        if let Some(record) = name_table.get_full_name() {
            println!("Full Name: Platform {}, Encoding {}, Language {:#x}", 
                     record.platform_id, record.encoding_id, record.language_id);
        }
    }
    
    if let Ok(head) = font.head_table() {
        println!("Font Revision: {:.2}", head.font_revision);
        println!("Units per EM: {}", head.units_per_em);
    }
    
    if let Ok(hhea) = font.hhea_table() {
        println!("Ascent: {}", hhea.ascent);
        println!("Descent: {}", hhea.descent);
        println!("Line Gap: {}", hhea.line_gap);
    }
    
    if let Ok(os2) = font.os2_table() {
        println!("Embedding Type: {:#x}", os2.fs_type);
    }
    
    println!("\n=== Modifying Font ===");
    
    let mut modifier = font.modify();
    modifier.set_font_name("Modified Font Family")?;
    modifier.set_full_font_name("Modified Font Family Regular")?;
    modifier.set_version(2, 0)?;
    modifier.set_copyright("Copyright (c) 2026 Modified Font")?;
    modifier.set_trademark("Modified Font is a trademark")?;
    modifier.set_font_revision(2, 5)?;
    modifier.set_embedding_type(0x0000)?; // Installable embedding
    modifier.set_localized_font_name("修改字体", 0x0804)?; // Simplified Chinese
    modifier.set_font_metrics(1000, 800, -200, 100)?;
    let modified_font = modifier.commit()?;
    
    println!("Font modified successfully!");
    
    println!("\n=== Modified Font Information ===");
    
    if let Ok(name_table) = modified_font.name_table() {
        println!("Name records: {}", name_table.name_records.len());
        for (idx, record) in name_table.name_records.iter().take(10).enumerate() {
            println!("  Record {}: Name ID {}, Platform {}, Encoding {}, Language {:#x}", 
                     idx, record.name_id, record.platform_id, 
                     record.encoding_id, record.language_id);
        }
    }
    
    if let Ok(head) = modified_font.head_table() {
        println!("Font Revision: {:.2}", head.font_revision);
        println!("Units per EM: {}", head.units_per_em);
    }
    
    if let Ok(hhea) = modified_font.hhea_table() {
        println!("Ascent: {}", hhea.ascent);
        println!("Descent: {}", hhea.descent);
        println!("Line Gap: {}", hhea.line_gap);
    }
    
    if let Ok(os2) = modified_font.os2_table() {
        println!("Embedding Type: {:#x}", os2.fs_type);
    }
    
    // Save the modified font
    let output_path = "modified_font.ttf";
    println!("\n=== Saving Modified Font ===");
    println!("Saving to: {}", output_path);
    
    modified_font.save(output_path)?;
    println!("Font saved successfully!");
    
    // Verify the saved font can be loaded
    println!("\n=== Verifying Saved Font ===");
    let verified_font = Font::load(output_path)?;
    
    if let Ok(head) = verified_font.head_table() {
        println!("Verified Font Revision: {:.2}", head.font_revision);
        println!("Verified Units per EM: {}", head.units_per_em);
    }
    
    println!("\nFont modification example completed successfully!");
    
    Ok(())
}
