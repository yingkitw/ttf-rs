// CLI tool to inspect TTF font files
use std::env;
use std::path::Path;
use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <font.ttf>", args[0]);
        eprintln!();
        eprintln!("Display information about a TrueType font file.");
        std::process::exit(1);
    }

    let font_path = &args[1];
    let font = Font::load(font_path)?;

    println!("Font: {}", Path::new(font_path).display());
    println!();

    // Basic information
    println!("Basic Information:");
    println!("  SFNT Version: {:#x}", font.sfnt_version);
    println!("  Number of tables: {}", font.num_tables);
    println!();

    // Font names
    if let Ok(name) = font.font_name() {
        println!("Font Name: {}", name);
    }
    if let Ok(family) = font.family_name() {
        println!("Family Name: {}", family);
    }
    println!();

    // Font metrics
    if let Ok(units_per_em) = font.units_per_em() {
        println!("  Units per EM: {}", units_per_em);
    }
    if let Ok(num_glyphs) = font.num_glyphs() {
        println!("  Number of glyphs: {}", num_glyphs);
    }
    if let Ok(is_bold) = font.is_bold() {
        println!("  Bold: {}", is_bold);
    }
    if let Ok(is_italic) = font.is_italic() {
        println!("  Italic: {}", is_italic);
    }
    println!();

    // Tables
    println!("Tables:");
    let tables = font.list_tables();
    for table in &tables {
        println!("  {}", table);
    }

    Ok(())
}
