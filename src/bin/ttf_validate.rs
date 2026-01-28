// CLI tool to validate TTF font files
use std::env;
use std::path::Path;
use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <font.ttf>", args[0]);
        eprintln!();
        eprintln!("Validate a TrueType font file.");
        std::process::exit(1);
    }

    let font_path = &args[1];
    let font = Font::load(font_path)?;

    println!("Validating font: {}", Path::new(font_path).display());
    println!();

    let report = font.validate()?;

    print!("{}", report.summary());

    if !report.is_valid {
        std::process::exit(1);
    }

    Ok(())
}
