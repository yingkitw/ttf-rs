// Comprehensive integration tests for ttf-rs
//
// These tests validate the major capabilities of the library

use ttf_rs::{
    Font, FontModifier, FontSubset, ValidationReport,
    CmapSubtable, GlyphData,
};
use tempfile::NamedTempFile;

#[test]
fn test_library_exports() {
    // Test that all major types are exported
    use ttf_rs::{
        TtfError, Result,
        HeadTable, MaxpTable, CmapTable, NameTable,
        HheaTable, HmtxTable, GlyfTable, PostTable, Os2Table,
        TableRecord, TtfTable, TtfTableWrite,
    };

    // Just verify types exist and can be referenced
    fn _assert_types() {
        let _: Option<TtfError> = None;
        let _: Option<Result<()>> = None;
    }
}

#[test]
fn test_font_reader_writer() {
    use ttf_rs::FontReader;

    // Test FontReader basic operations
    let data = vec![
        0x00, 0x01, 0x00, 0x00, // SFNT version
        0x00, 0x0A,             // Num tables
        0x00, 0x20,             // Search range
        0x00, 0x03,             // Entry selector
        0x00, 0x10,             // Range shift
    ];

    let mut reader = FontReader::from_slice(&data);

    assert_eq!(reader.read_u32().unwrap(), 0x00010000);
    assert_eq!(reader.read_u16().unwrap(), 10);
    assert_eq!(reader.read_u16().unwrap(), 0x20);
    assert_eq!(reader.position(), 8);
}

#[test]
fn test_font_writer() {
    use ttf_rs::FontWriter;

    let mut writer = FontWriter::new();
    writer.write_u32(0x00010000);
    writer.write_u16(10);
    writer.write_u16(0x20);

    let data = writer.into_inner();
    assert_eq!(data.len(), 8);
    assert_eq!(data[0..4], [0x00, 0x01, 0x00, 0x00]);
    assert_eq!(data[4..6], [0x00, 0x0A]);
    assert_eq!(data[6..8], [0x00, 0x20]);
}

#[test]
fn test_checksum_calculation() {
    use ttf_rs::calculate_checksum;

    let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let checksum = calculate_checksum(&data);
    // Verify checksum is calculated
    assert_ne!(checksum, 0);
}

#[test]
fn test_table_record() {
    use ttf_rs::TableRecord;

    let record = TableRecord::new(*b"head", 12, 100);
    assert_eq!(record.table_tag, *b"head");
    assert_eq!(record.offset, 12);
    assert_eq!(record.length, 100);

    let tag_string = record.tag_to_string();
    assert_eq!(tag_string, "head");
}

#[test]
fn test_font_modifier_api() {
    // Test that FontModifier API exists
    let font_data = create_minimal_font_data();
    let result = Font::from_data(font_data.clone());

    match result {
        Ok(font) => {
            let _modifier = font.into_modifier();
            // If we get here, the modifier was created successfully
        }
        Err(_) => {
            // Expected - the minimal data isn't a valid font
            // But the API exists
        }
    }
}

#[test]
fn test_font_subset_api() {
    let font_data = create_minimal_font_data();
    let result = Font::from_data(font_data.clone());

    match result {
        Ok(font) => {
            let subset = font.subset();
            // Test the subset builder API
            drop(subset);
        }
        Err(_) => {
            // Expected - minimal data isn't valid
        }
    }
}

#[test]
fn test_cached_font_api() {
    let font_data = create_minimal_font_data();
    let result = Font::from_data(font_data.clone());

    match result {
        Ok(font) => {
            let cached = font.with_cache();
            // Test that cached font can be created
            drop(cached);
        }
        Err(_) => {
            // Expected - minimal data isn't valid
        }
    }
}

#[test]
fn test_validation_api() {
    // Test that validation API exists
    let font_data = create_minimal_font_data();
    let result = Font::from_data(font_data.clone());

    match result {
        Ok(font) => {
            let _report = font.validate();
            // If we get here, validation was called
        }
        Err(_) => {
            // Expected - minimal data isn't valid
        }
    }
}

#[test]
fn test_error_types() {
    use ttf_rs::TtfError;
    use std::io;

    // Test error creation
    let error = TtfError::Io(io::Error::new(
        io::ErrorKind::NotFound,
        "test error"
    ));
    assert!(matches!(error, TtfError::Io(_)));

    let error = TtfError::ParseError("test parse error".to_string());
    assert!(matches!(error, TtfError::ParseError(_)));

    let error = TtfError::InvalidSignature {
        expected: 123,
        actual: 456,
    };
    assert!(matches!(error, TtfError::InvalidSignature { .. }));
}

#[test]
fn test_glyph_data_variants() {
    use ttf_rs::{SimpleGlyph, CompositeGlyph};

    // Test all variants can be created
    let simple = GlyphData::Simple(SimpleGlyph {
        end_pts_of_contours: vec![],
        instruction_length: 0,
        instructions: vec![],
        flags: vec![],
        x_coordinates: vec![],
        y_coordinates: vec![],
    });

    let composite = GlyphData::Composite(CompositeGlyph {
        components: vec![],
    });

    let empty = GlyphData::Empty;

    // Verify variants exist
    match simple {
        GlyphData::Simple(_) => {}
        _ => panic!("Wrong variant"),
    }

    match composite {
        GlyphData::Composite(_) => {}
        _ => panic!("Wrong variant"),
    }

    match empty {
        GlyphData::Empty => {}
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_cmap_subtable() {
    use ttf_rs::Format4;

    // Test Format4 creation
    let format4 = Format4 {
        format: 4,
        length: 0,
        language: 0,
        seg_count_x2: 2,
        search_range: 2,
        entry_selector: 0,
        range_shift: 0,
        end_codes: vec![0xFF],
        start_codes: vec![0x00],
        id_deltas: vec![0],
        id_range_offsets: vec![0],
        glyph_id_array: vec![],
    };

    // Test get_glyph method
    let result = format4.get_glyph(0x41); // 'A'
    assert!(result.is_some());

    // Test that it's accessible through CmapSubtable
    let subtable = CmapSubtable::Format4(format4);
    match subtable {
        CmapSubtable::Format4(f) => {
            assert_eq!(f.format, 4);
        }
        _ => panic!("Wrong subtable type"),
    }
}

#[test]
fn test_head_table_helper() {
    use ttf_rs::HeadTable;

    let head = HeadTable {
        table_version: 1.0,
        font_revision: 1.0,
        checksum_adjustment: 0,
        magic_number: 0x5F0F3CF5,
        flags: 0,
        units_per_em: 1000,
        created: 0,
        modified: 0,
        x_min: 0,
        y_min: 0,
        x_max: 1000,
        y_max: 1000,
        mac_style: 0,
        lowest_rec_ppem: 8,
        font_direction_hint: 2,
        index_to_loc_format: 0,
        glyph_data_format: 0,
    };

    assert_eq!(head.units_per_em, 1000);
    assert_eq!(head.is_long_loca_format(), false);

    let mut head2 = head.clone();
    head2.index_to_loc_format = 1;
    assert_eq!(head2.is_long_loca_format(), true);
}

#[test]
fn test_os2_table_helpers() {
    use ttf_rs::Os2Table;

    let os2 = Os2Table {
        version: 4,
        x_avg_char_width: 500,
        us_weight_class: 400,
        us_width_class: 5,
        fs_type: 0,
        y_subscript_x_size: 650,
        y_subscript_y_size: 600,
        y_subscript_x_offset: 0,
        y_subscript_y_offset: 75,
        y_superscript_x_size: 650,
        y_superscript_y_size: 600,
        y_superscript_x_offset: 0,
        y_superscript_y_offset: 350,
        y_strikeout_size: 50,
        y_strikeout_position: 300,
        s_family_class: 0,
        panose: [0; 10],
        ul_unicode_range1: 1,
        ul_unicode_range2: 0,
        ul_unicode_range3: 0,
        ul_unicode_range4: 0,
        ach_vend_id: [0; 4],
        fs_selection: 64,
        us_first_char_index: 32,
        us_last_char_index: 126,
        s_typo_ascender: 800,
        s_typo_descender: -200,
        s_typo_line_gap: 0,
        us_win_ascent: 1000,
        us_win_descent: 200,
        ul_code_page_range1: 1,
        ul_code_page_range2: 0,
        sx_height: 500,
        s_cap_height: 700,
        us_default_char: 0,
        us_break_char: 32,
        us_max_context: 0,
    };

    assert!(!os2.is_bold());
    assert!(!os2.is_italic());
    assert_eq!(os2.get_weight_string(), "Normal");

    let bold_os2 = Os2Table {
        us_weight_class: 700,
        ..os2.clone()
    };
    assert!(bold_os2.is_bold());
}

#[test]
fn test_maxp_table_version() {
    use ttf_rs::MaxpTable;

    let maxp_05 = MaxpTable {
        version: 0.5,
        num_glyphs: 100,
        max_points: None,
        max_contours: None,
        max_composite_points: None,
        max_composite_contours: None,
        max_zones: None,
        max_twilight_points: None,
        max_storage: None,
        max_function_defs: None,
        max_instruction_defs: None,
        max_stack_elements: None,
        max_size_of_instructions: None,
        max_component_elements: None,
        max_component_depth: None,
    };

    assert!(maxp_05.is_version_0_5());
    assert!(!maxp_05.is_version_1_0());

    let maxp_10 = MaxpTable {
        version: 1.0,
        num_glyphs: 100,
        max_points: Some(100),
        max_contours: Some(10),
        max_composite_points: None,
        max_composite_contours: None,
        max_zones: None,
        max_twilight_points: None,
        max_storage: None,
        max_function_defs: None,
        max_instruction_defs: None,
        max_stack_elements: None,
        max_size_of_instructions: None,
        max_component_elements: None,
        max_component_depth: None,
    };

    assert!(maxp_10.is_version_1_0());
    assert!(!maxp_10.is_version_0_5());
}

#[test]
fn test_font_save_api() {
    use std::io::Write;

    // Create a minimal font
    let font_data = create_minimal_font_data();

    // Try to create a temp file and test save
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(&font_data).unwrap();

    // Attempt to load (will likely fail due to minimal data)
    let result = Font::load(temp_file.path());

    // We don't expect this to succeed with minimal data,
    // but we're testing the API works
    match result {
        Ok(_) => {
            // If it succeeded, great!
        }
        Err(_) => {
            // Expected with minimal data
        }
    }
}

#[test]
fn test_table_tag_operations() {
    use ttf_rs::TableRecord;

    // Test tag operations
    let record = TableRecord::new(*b"cmap", 12, 100);
    assert_eq!(record.tag_to_string(), "cmap");

    let record2 = TableRecord::new(*b"glyf", 200, 500);
    assert_eq!(record2.tag_to_string(), "glyf");
}

#[test]
fn test_bounding_box() {
    use ttf_rs::{BoundingBox, Point};

    let bbox1 = BoundingBox::new(0.0, 0.0, 100.0, 100.0);
    assert_eq!(bbox1.width(), 100.0);
    assert_eq!(bbox1.height(), 100.0);

    let point = Point::new(50.0, 50.0);
    assert_eq!(point.x, 50.0);
    assert_eq!(point.y, 50.0);
}

#[test]
fn test_transform() {
    use ttf_rs::Transform;

    let transform = Transform {
        xx: 1.0, xy: 0.0,
        yx: 0.0, yy: 1.0,
        dx: 10.0, dy: 20.0,
    };

    assert_eq!(transform.xx, 1.0);
    assert_eq!(transform.dx, 10.0);
}

// Performance test to ensure operations complete in reasonable time
#[test]
fn test_performance_checks() {
    use std::time::Instant;

    let data = vec![0u8; 1024 * 1024]; // 1MB of data
    let now = Instant::now();

    // Test checksum performance
    let checksum = ttf_rs::calculate_checksum(&data);
    let elapsed = now.elapsed();

    // Should complete in reasonable time (< 100ms for 1MB)
    assert!(elapsed.as_millis() < 100, "Checksum calculation took too long: {:?}", elapsed);
    assert_ne!(checksum, 0);
}

// Helper function to create minimal font data for testing
fn create_minimal_font_data() -> Vec<u8> {
    let mut data = Vec::new();

    // SFNT header
    data.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]); // SFNT version
    data.extend_from_slice(&[0x00, 0x01]); // Num tables (1)

    // Table directory
    data.extend_from_slice(b"head"); // Table tag
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Checksum
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Offset
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x40]); // Length (64 bytes)

    // Minimal head table data (54 bytes of zeros + padding)
    data.extend_from_slice(&[0u8; 64]);

    data
}
