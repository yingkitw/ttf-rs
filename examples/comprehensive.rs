// Comprehensive example showcasing ttf-rs capabilities
//
// This example demonstrates:
// - Font loading and inspection
// - Table parsing (head, maxp, cmap, name, glyf, etc.)
// - Character to glyph mapping
// - Font modification
// - Font subsetting
// - Font validation
// - Metrics reporting
// - Caching
// - Format conversion

use ttf_rs::{Font, FontModifier, FontSubset, ValidationReport, CachedFont};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ttf-rs Comprehensive Example ===\n");

    // Note: This example requires a font file to work with
    // For demonstration, we'll show what operations are available

    println!("This example demonstrates the following capabilities:\n");

    println!("1. FONT LOADING");
    println!("   - Load from file: Font::load(path)");
    println!("   - Load from bytes: Font::from_data(bytes)");
    println!("   - Supported formats: TTF, OTF, WOFF (basic)");
    println!();

    println!("2. TABLE INSPECTION");
    println!("   Available tables:");
    println!("   - head: Font header (version, bounding box, units per em)");
    println!("   - maxp: Maximum profile (glyph count, contour limits)");
    println!("   - cmap: Character to glyph mapping");
    println!("   - name: Font naming (family, style, copyright)");
    println!("   - hhea: Horizontal header (ascender, descender, line gap)");
    println!("   - hmtx: Horizontal metrics (advance widths, side bearings)");
    println!("   - glyf: Glyph data (outlines, composite glyphs)");
    println!("   - loca: Index to location");
    println!("   - post: PostScript information");
    println!("   - OS/2: OS/2 and Windows metrics");
    println!("   - cmap formats: 0, 4, 6, 12, 13, 14");
    println!("   - Additional tables: kern, GPOS, GSUB, BASE, JSTF");
    println!("   - Variable fonts: fvar, gvar, avar, STAT, HVAR/VVAR");
    println!("   - Color fonts: COLR, CPAL, SVG, CBDT/CBLC, sbix");
    println!("   - Instructions: fpgm, prep, cvt");
    println!();

    println!("3. CHARACTER TO GLYPH MAPPING");
    println!("   - Get glyph ID: font.char_to_glyph('A')");
    println!("   - Supports Unicode BMP and full repertoire");
    println!("   - Multiple cmap formats supported");
    println!();

    println!("4. FONT MODIFICATION");
    println!("   Using FontModifier:");
    println!("   - Set font family name");
    println!("   - Set font version");
    println!("   - Modify font metrics (ascender, descender, line gap)");
    println!("   - Modify glyph advance widths");
    println!("   - Set embedding permissions");
    println!("   - Save modified font: modifier.save(path)");
    println!();

    println!("5. FONT SUBSETTING");
    println!("   Using FontSubset:");
    println!("   - Subset by character: font.subset().with_chars(&['A', 'B', 'C'])");
    println!("   - Subset by glyph ID: font.subset().with_glyphs(&[0, 1, 2])");
    println!("   - Retain specific tables: subset.retain_tables(&[b\"head\", b\"cmap\"])");
    println!("   - Build subset: subset.build()");
    println!("   - Saves space by including only needed glyphs");
    println!("   - Automatically updates cmap, glyf, loca, hmtx, maxp tables");
    println!();

    println!("6. FONT VALIDATION");
    println!("   Using ValidationReport:");
    println!("   - Validate table checksums");
    println!("   - Check required tables present");
    println!("   - Validate table structure");
    println!("   - Check glyph data integrity");
    println!("   - Report errors and warnings");
    println!();

    println!("7. PERFORMANCE OPTIMIZATION");
    println!("   Using CachedFont:");
    println!("   - Lazy table loading");
    println!("   - Table data caching");
    println!("   - Thread-safe with Arc<RwLock>");
    println!("   - Use for repeated table access: font.with_cache()");
    println!();

    println!("8. FORMAT CONVERSION");
    println!("   - TTF to WOFF: font.to_woff()");
    println!("   - TTF to WOFF2: font.to_woff2()");
    println!("   - (Full implementation requires compression libraries)");
    println!();

    println!("9. CLI TOOLS");
    println!("   - ttf-info: Display font information");
    println!("   - ttf-validate: Validate font file");
    println!("   - ttf-subset: Create font subset");
    println!("   - ttf-metrics: Display comprehensive metrics");
    println!();

    println!("=== Example Code ===\n");

    // Show example code snippets
    let example_code = r#"
// Load a font
let font = Font::load("path/to/font.ttf")?;

// Get font information
let head = font.head_table()?;
println!("Font version: {}", head.font_revision);
println!("Units per em: {}", head.units_per_em);

// Character to glyph mapping
let glyph_id = font.char_to_glyph('A')?;
println!("Glyph ID for 'A': {}", glyph_id);

// Get glyph data
let glyf = font.glyf_table()?;
if let Some(glyph) = glyf.get_glyph(glyph_id as usize) {
    println!("Glyph contours: {}", glyph.number_of_contours);

    // Calculate bounding box
    if let Some(bbox) = glyph.calculate_bounding_box() {
        println!("Bounding box: {:?}", bbox);
    }
}

// Modify font
let mut modifier = font.into_modifier();
modifier.set_font_name("My Custom Font")?;
modifier.set_version(1, 0)?;
modifier.set_font_metrics(1000, -200, 0)?;
modifier.save("output/custom_font.ttf")?;

// Subset font
let subset = font.subset()
    .with_chars(&['A', 'B', 'C', '0', '1', '2'])?
    .retain_tables(&[b"head", b"cmap", b"glyf", b"loca", b"hmtx"]);
let subset_font = subset.build()?;
subset_font.save("output/subset.ttf")?;

// Validate font
let report = ValidationReport::validate(&font)?;
if !report.is_valid {
    eprintln!("Validation errors:");
    for error in &report.errors {
        eprintln!("  - {:?}", error);
    }
}

// Use caching for performance
let cached = font.with_cache();
let head = cached.head_table_cached()?;
let maxp = cached.maxp_table_cached()?;
let cmap = cached.cmap_table_cached()?;

// Convert to WOFF
let woff_data = font.to_woff()?;
std::fs::write("output/font.woff", woff_data)?;
"#;

    println!("{}", example_code);

    println!("\n=== Advanced Features ===\n");

    println!("VARIABLE FONTS:");
    println!("   - Parse fvar table for font variations");
    println!("   - Parse gvar table for glyph variations");
    println!("   - Parse avar table for axis variations");
    println!("   - Parse STAT table for style attributes");
    println!("   - Parse HVAR/VVAR tables for metric variations");
    println!();

    println!("COLOR FONTS:");
    println!("   - COLR table: Color layers");
    println!("   - CPAL table: Color palettes");
    println!("   - SVG table: SVG glyph descriptions");
    println!("   - CBDT/CBLC tables: Embedded bitmaps");
    println!("   - sbix table: Standard bitmap graphics");
    println!();

    println!("ADVANCED GLYPH OPERATIONS:");
    println!("   - Calculate bounding boxes");
    println!("   - Transform glyphs (scale, rotate, translate)");
    println!("   - Simplify outlines");
    println!("   - Resolve composite glyphs");
    println!();

    println!("RASTERIZATION:");
    println!("   - Outline to bitmap conversion");
    println!("   - Anti-aliasing support");
    println!("   - Glyph caching");
    println!();

    println!("=== Testing ===\n");
    println!("Run tests with: cargo test");
    println!("Run examples with: cargo run --example <example-name>");
    println!();

    Ok(())
}
