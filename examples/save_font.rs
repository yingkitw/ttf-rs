// Example of saving and modifying TTF fonts
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

    println!("=== Font Saving Example ===");
    println!();

    // Display original font info
    if let Ok(head) = font.head_table() {
        println!("Original font:");
        println!("  Version: {}", head.table_version);
        println!("  Units per EM: {}", head.units_per_em);
    }
    println!();

    // Save the font to a new file
    println!("Saving font to 'output.ttf'...");
    match font.save("output.ttf") {
        Ok(_) => println!("Font saved successfully!"),
        Err(e) => eprintln!("Error saving font: {}", e),
    }

    println!();

    // Demonstrate font modification
    println!("=== Font Modification Example ===");
    println!();

    // Get table data
    if let Some(head_data) = font.get_table_data(b"head") {
        println!("head table size: {} bytes", head_data.len());

        // In a real application, you would:
        // 1. Parse the table data
        // 2. Modify specific fields
        // 3. Serialize it back
        // 4. Create a modified font

        println!("In a real application, you could:");
        println!("  - Modify font metrics");
        println!("  - Update naming table");
        println!("  - Remove glyphs (subsetting)");
        println!("  - Adjust character mappings");
    }

    println!();

    // Demonstrate round-trip: load the saved font
    println!("=== Verifying Saved Font ===");
    println!();

    match Font::load("output.ttf") {
        Ok(loaded_font) => {
            println!("Successfully reloaded saved font!");
            println!("Number of tables: {}", loaded_font.num_tables);

            if let Ok(head) = loaded_font.head_table() {
                println!("Units per EM: {}", head.units_per_em);
            }

            println!("Tables:");
            for table in loaded_font.list_tables() {
                println!("  - {}", table);
            }
        }
        Err(e) => eprintln!("Error loading saved font: {}", e),
    }
}
