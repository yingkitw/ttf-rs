// CLI tool to subset TTF font files
use std::env;
use std::path::Path;
use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <input.ttf> <output.ttf> <chars>", args[0]);
        eprintln!();
        eprintln!("Subset a TrueType font file to include only the specified characters.");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} input.ttf output.ttf ABCabc", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let chars_string = &args[3];

    let font = Font::load(input_path)?;

    println!("Loading font: {}", Path::new(input_path).display());

    // Parse characters
    let chars: Vec<char> = chars_string.chars().collect();

    println!("Creating subset with {} characters...", chars.len());

    let mut subset = font.subset();
    subset.with_chars(&chars)?;

    let subset_font = subset.build()?;

    println!("Saving subset to: {}", Path::new(output_path).display());
    subset_font.save(output_path)?;

    println!("Done!");

    Ok(())
}
