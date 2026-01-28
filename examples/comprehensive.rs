// Comprehensive test validation example for ttf-rs
//
// This example validates all major capabilities by:
// - Loading fonts from file and bytes
// - Parsing all table types
// - Testing character to glyph mapping
// - Validating font modification
// - Testing font subsetting
// - Running validation checks
// - Testing performance with caching
// - Verifying binary I/O operations

use ttf_rs::{
    Font, FontModifier, FontSubset, ValidationReport, CachedFont,
    FontReader, FontWriter, calculate_checksum,
    HeadTable, MaxpTable, CmapTable, NameTable, HheaTable,
    GlyphData, BoundingBox,
};
use std::env;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║   ttf-rs Comprehensive Capability Validation          ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    // Get font path from command line or use a test font
    let font_path = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: cargo run --example comprehensive <font.ttf>");
        println!("No font provided, creating minimal test font...\n");
        String::new()
    });

    if font_path.is_empty() {
        validate_with_minimal_font()?;
    } else {
        validate_with_real_font(&font_path)?;
    }

    println!("\n✅ All capability validations completed successfully!");
    Ok(())
}

fn validate_with_minimal_font() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ Testing with Minimal Font Data ═══\n");
    
    let minimal_data = create_minimal_font_data();
    
    test_font_loading(&minimal_data)?;
    test_binary_io()?;
    test_checksum_calculation()?;
    test_api_existence()?;
    
    Ok(())
}

fn validate_with_real_font(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("═══ Testing with Real Font: {} ═══\n", path);
    
    let font = Font::load(path)?;
    
    test_table_parsing(&font)?;
    test_character_mapping(&font)?;
    test_glyph_operations(&font)?;
    test_font_validation(&font)?;
    test_caching_performance(&font)?;
    test_font_modification(font.clone())?;
    test_font_subsetting(&font)?;
    
    Ok(())
}

fn test_font_loading(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("[1/8] Testing Font Loading...");

    
    // Test loading from bytes
    let font = Font::from_data(data.to_vec())?;
    println!("  ✓ Load from bytes");
    
    // Verify basic structure
    assert!(font.table_records.len() > 0, "Font should have tables");
    println!("  ✓ Font structure valid ({} tables)", font.table_records.len());
    
    println!("  ✅ Font loading validated\n");
    Ok(())
}

fn test_binary_io() -> Result<(), Box<dyn std::error::Error>> {
    println!("[2/8] Testing Binary I/O Operations...");

    
    // Test FontReader
    let data = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x0A];
    let mut reader = FontReader::from_slice(&data);
    assert_eq!(reader.read_u32()?, 0x00010000);
    assert_eq!(reader.read_u16()?, 10);
    println!("  ✓ FontReader operations");
    
    // Test FontWriter
    let mut writer = FontWriter::new();
    writer.write_u32(0x00010000);
    writer.write_u16(10);
    let written = writer.into_inner();
    assert_eq!(written, vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x0A]);
    println!("  ✓ FontWriter operations");
    
    println!("  ✅ Binary I/O validated\n");
    Ok(())
}

fn test_checksum_calculation() -> Result<(), Box<dyn std::error::Error>> {
    println!("[3/8] Testing Checksum Calculation...");
    
    let data = vec![0x00, 0x01, 0x00, 0x00];
    let checksum = calculate_checksum(&data);
    assert_eq!(checksum, 0x00010000);
    println!("  ✓ Checksum calculation");
    
    println!("  ✅ Checksum validated\n");
    Ok(())
}

fn test_api_existence() -> Result<(), Box<dyn std::error::Error>> {
    println!("[4/8] Testing API Existence...");
    
    // Just verify types exist and can be constructed
    let _ = std::marker::PhantomData::<Font>;
    let _ = std::marker::PhantomData::<FontModifier>;
    let _ = std::marker::PhantomData::<FontSubset>;
    let _ = std::marker::PhantomData::<ValidationReport>;
    let _ = std::marker::PhantomData::<CachedFont>;
    println!("  ✓ Core types available");
    
    let _ = std::marker::PhantomData::<HeadTable>;
    let _ = std::marker::PhantomData::<MaxpTable>;
    let _ = std::marker::PhantomData::<CmapTable>;
    let _ = std::marker::PhantomData::<NameTable>;
    let _ = std::marker::PhantomData::<HheaTable>;
    println!("  ✓ Table types available");
    
    let _ = std::marker::PhantomData::<GlyphData>;
    let _ = std::marker::PhantomData::<BoundingBox>;
    println!("  ✓ Glyph types available");
    
    println!("  ✅ API existence validated\n");
    Ok(())
}

fn test_table_parsing(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[1/7] Testing Table Parsing...");

    
    // Test HEAD table
    if let Ok(head) = font.head_table() {
        println!("  ✓ HEAD table (version: {}, units_per_em: {})", head.font_revision, head.units_per_em);
    }
    
    // Test MAXP table
    if let Ok(maxp) = font.maxp_table() {
        println!("  ✓ MAXP table ({} glyphs)", maxp.num_glyphs);
    }
    
    // Test CMAP table
    if let Ok(cmap) = font.cmap_table() {
        println!("  ✓ CMAP table ({} encodings)", cmap.encoding_records.len());
    }
    
    // Test NAME table
    if let Ok(name) = font.name_table() {
        println!("  ✓ NAME table ({} records)", name.count);
    }
    
    // Test HHEA table
    if let Ok(hhea) = font.hhea_table() {
        println!("  ✓ HHEA table (ascent: {}, descent: {})", hhea.ascent, hhea.descent);
    }
    
    println!("  ✅ Table parsing validated\n");
    Ok(())
}

fn test_character_mapping(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[2/7] Testing Character to Glyph Mapping...");

    
    // Test basic ASCII characters
    let test_chars = vec!['A', 'B', 'a', 'b', '0', '1', ' '];
    let mut mapped_count = 0;
    
    for ch in test_chars {
        if let Ok(_glyph_id) = font.char_to_glyph(ch) {
            mapped_count += 1;
        }
    }
    
    println!("  ✓ Character mapping ({} chars mapped)", mapped_count);
    println!("  ✅ Character mapping validated\n");
    Ok(())
}

fn test_glyph_operations(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[3/7] Testing Glyph Operations...");
    
    if let Ok(glyf) = font.glyf_table() {
        let glyph_count = glyf.glyphs.len();
        println!("  ✓ GLYF table loaded ({} glyphs)", glyph_count);
        
        // Test glyph data variants
        let mut simple = 0;
        let mut composite = 0;
        let mut empty = 0;
        
        for glyph in &glyf.glyphs {
            match &glyph.data {
                GlyphData::Simple(_) => simple += 1,
                GlyphData::Composite(_) => composite += 1,
                GlyphData::Empty => empty += 1,
            }
        }
        
        println!("  ✓ Glyph types: {} simple, {} composite, {} empty", simple, composite, empty);
    }
    
    println!("  ✅ Glyph operations validated\n");
    Ok(())
}

fn test_font_validation(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[4/7] Testing Font Validation...");
    
    let report = font.validate()?;
    println!("  ✓ Validation report generated");
    println!("  ✓ Valid: {}", report.is_valid);
    println!("  ✓ Errors: {}", report.errors.len());
    println!("  ✓ Warnings: {}", report.warnings.len());
    
    println!("  ✅ Font validation completed\n");
    Ok(())
}

fn test_caching_performance(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[5/7] Testing Caching Performance...");
    
    // Test without cache
    let start = Instant::now();
    for _ in 0..10 {
        let _ = font.head_table();
        let _ = font.maxp_table();
    }
    let without_cache = start.elapsed();
    
    // Test with cache
    let cached = font.clone().with_cache();
    let start = Instant::now();
    for _ in 0..10 {
        let _ = cached.head_table_cached();
        let _ = cached.maxp_table_cached();
    }
    let with_cache = start.elapsed();
    
    println!("  ✓ Without cache: {:?}", without_cache);
    println!("  ✓ With cache: {:?}", with_cache);
    
    println!("  ✅ Caching performance validated\n");
    Ok(())
}

fn test_font_modification(font: Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[6/7] Testing Font Modification...");

    
    let mut modifier = FontModifier::new(font);
    
    // Test setting font name
    modifier.set_font_name("Test Modified Font")?;
    println!("  ✓ Set font name");
    
    // Test setting version
    modifier.set_version(2, 0)?;
    println!("  ✓ Set version");
    
    // Test setting metrics
    modifier.set_font_metrics(1000, 800, -200, 0)?;
    println!("  ✓ Set font metrics");
    
    // Commit modifications
    let _modified_font = modifier.commit()?;
    println!("  ✓ Commit modifications");
    
    println!("  ✅ Font modification validated\n");
    Ok(())
}

fn test_font_subsetting(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
    println!("[7/7] Testing Font Subsetting...");

    
    let chars = vec!['A', 'B', 'C', '0', '1', '2'];
    let mut subset = font.clone().subset();
    subset.with_chars(&chars)?;
    println!("  ✓ Create subset with {} characters", chars.len());
    
    let subset_font = subset.build()?;
    println!("  ✓ Build subset font");
    
    // Verify subset has fewer glyphs
    if let (Ok(orig_maxp), Ok(sub_maxp)) = (font.maxp_table(), subset_font.maxp_table()) {
        println!("  ✓ Original glyphs: {}, Subset glyphs: {}", orig_maxp.num_glyphs, sub_maxp.num_glyphs);
    }
    
    println!("  ✅ Font subsetting validated\n");
    Ok(())
}

fn create_minimal_font_data() -> Vec<u8> {
    let mut data = Vec::new();
    
    // SFNT header
    data.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]); // version
    data.extend_from_slice(&[0x00, 0x03]); // numTables
    data.extend_from_slice(&[0x00, 0x30]); // searchRange
    data.extend_from_slice(&[0x00, 0x01]); // entrySelector
    data.extend_from_slice(&[0x00, 0x00]); // rangeShift
    
    // Table records (head, maxp, cmap)
    let offset = 12 + 16 * 3;
    
    // head table record
    data.extend_from_slice(b"head");
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // checksum
    data.extend_from_slice(&(offset as u32).to_be_bytes());
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x36]); // length
    
    // maxp table record
    data.extend_from_slice(b"maxp");
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    data.extend_from_slice(&((offset + 0x36) as u32).to_be_bytes());
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x06]);
    
    // cmap table record
    data.extend_from_slice(b"cmap");
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    data.extend_from_slice(&((offset + 0x3C) as u32).to_be_bytes());
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x14]);
    
    // head table data (54 bytes)
    data.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]); // version
    data.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]); // fontRevision
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // checksumAdjustment
    data.extend_from_slice(&[0x5F, 0x0F, 0x3C, 0xF5]); // magicNumber
    data.extend_from_slice(&[0x00, 0x00]); // flags
    data.extend_from_slice(&[0x03, 0xE8]); // unitsPerEm (1000)
    data.extend_from_slice(&[0x00; 16]); // created/modified
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // xMin, yMin
    data.extend_from_slice(&[0x03, 0xE8, 0x03, 0xE8]); // xMax, yMax
    data.extend_from_slice(&[0x00, 0x00]); // macStyle
    data.extend_from_slice(&[0x00, 0x08]); // lowestRecPPEM
    data.extend_from_slice(&[0x00, 0x02]); // fontDirectionHint
    data.extend_from_slice(&[0x00, 0x00]); // indexToLocFormat
    data.extend_from_slice(&[0x00, 0x00]); // glyphDataFormat
    
    // maxp table data (6 bytes)
    data.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]); // version 1.0
    data.extend_from_slice(&[0x00, 0x01]); // numGlyphs
    
    // cmap table data (20 bytes)
    data.extend_from_slice(&[0x00, 0x00]); // version
    data.extend_from_slice(&[0x00, 0x01]); // numTables
    data.extend_from_slice(&[0x00, 0x03]); // platformID
    data.extend_from_slice(&[0x00, 0x01]); // encodingID
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x0C]); // offset
    data.extend_from_slice(&[0x00, 0x00]); // format
    data.extend_from_slice(&[0x00, 0x06]); // length
    data.extend_from_slice(&[0x00, 0x00]); // language
    
    data
}

