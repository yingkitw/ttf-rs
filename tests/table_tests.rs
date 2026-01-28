use ttf_rs::{FontReader, FontWriter};

#[test]
fn test_head_table_parse() {
    let mut writer = FontWriter::new();
    
    writer.write_fixed(1.0);
    writer.write_fixed(1.0);
    writer.write_u32(0);
    writer.write_u32(0x5F0F3CF5);
    writer.write_u16(0);
    writer.write_u16(1000);
    writer.write_i64(0);
    writer.write_i64(0);
    writer.write_i16(-100);
    writer.write_i16(-200);
    writer.write_i16(1000);
    writer.write_i16(1200);
    writer.write_u16(0);
    writer.write_u16(16);
    writer.write_i16(2);
    writer.write_i16(1);
    writer.write_i16(0);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let version = reader.read_fixed().unwrap();
    assert!((version - 1.0).abs() < 0.0001);
    
    let font_revision = reader.read_fixed().unwrap();
    assert!((font_revision - 1.0).abs() < 0.0001);
    
    reader.read_u32().unwrap();
    
    let magic = reader.read_u32().unwrap();
    assert_eq!(magic, 0x5F0F3CF5);
    
    reader.read_u16().unwrap();
    
    let units_per_em = reader.read_u16().unwrap();
    assert_eq!(units_per_em, 1000);
}

#[test]
fn test_maxp_table_parse() {
    let mut writer = FontWriter::new();
    
    writer.write_fixed(1.0);
    writer.write_u16(100);
    writer.write_u16(50);
    writer.write_u16(10);
    writer.write_u16(100);
    writer.write_u16(20);
    writer.write_u16(2);
    writer.write_u16(0);
    writer.write_u16(64);
    writer.write_u16(10);
    writer.write_u16(10);
    writer.write_u16(256);
    writer.write_u16(512);
    writer.write_u16(2);
    writer.write_u16(16);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let version = reader.read_fixed().unwrap();
    assert!((version - 1.0).abs() < 0.0001);
    
    let num_glyphs = reader.read_u16().unwrap();
    assert_eq!(num_glyphs, 100);
    
    let max_points = reader.read_u16().unwrap();
    assert_eq!(max_points, 50);
}

#[test]
fn test_hhea_table_parse() {
    let mut writer = FontWriter::new();
    
    writer.write_fixed(1.0);
    writer.write_i16(800);
    writer.write_i16(-200);
    writer.write_i16(100);
    writer.write_u16(1000);
    writer.write_i16(-50);
    writer.write_i16(-50);
    writer.write_i16(1000);
    writer.write_i16(1);
    writer.write_i16(0);
    writer.write_i16(0);
    writer.write_i16(0);
    writer.write_i16(0);
    writer.write_i16(0);
    writer.write_i16(0);
    writer.write_i16(0);
    writer.write_u16(100);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let version = reader.read_fixed().unwrap();
    assert!((version - 1.0).abs() < 0.0001);
    
    let ascent = reader.read_i16().unwrap();
    assert_eq!(ascent, 800);
    
    let descent = reader.read_i16().unwrap();
    assert_eq!(descent, -200);
    
    let line_gap = reader.read_i16().unwrap();
    assert_eq!(line_gap, 100);
}

#[test]
fn test_cmap_format_0() {
    let mut writer = FontWriter::new();
    
    writer.write_u16(0);
    writer.write_u16(262);
    writer.write_u16(0);
    
    for i in 0..=255u8 {
        writer.write_u8(i);
    }
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let format = reader.read_u16().unwrap();
    assert_eq!(format, 0);
    
    let length = reader.read_u16().unwrap();
    assert_eq!(length, 262);
    
    let language = reader.read_u16().unwrap();
    assert_eq!(language, 0);
    
    let glyph_0 = reader.read_u8().unwrap();
    assert_eq!(glyph_0, 0);
    
    reader.set_position(6 + 65).unwrap();
    let glyph_65 = reader.read_u8().unwrap();
    assert_eq!(glyph_65, 65);
}

#[test]
fn test_post_table_parse() {
    let mut writer = FontWriter::new();
    
    writer.write_fixed(2.0);
    writer.write_fixed(0.0);
    writer.write_i16(-100);
    writer.write_i16(50);
    writer.write_u32(0);
    writer.write_u32(0);
    writer.write_u32(0);
    writer.write_u32(0);
    writer.write_u32(0);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let version = reader.read_fixed().unwrap();
    assert!((version - 2.0).abs() < 0.0001);
    
    let italic_angle = reader.read_fixed().unwrap();
    assert!((italic_angle - 0.0).abs() < 0.0001);
    
    let underline_position = reader.read_i16().unwrap();
    assert_eq!(underline_position, -100);
    
    let underline_thickness = reader.read_i16().unwrap();
    assert_eq!(underline_thickness, 50);
}

#[test]
fn test_name_record_parse() {
    let mut writer = FontWriter::new();
    
    writer.write_u16(3);
    writer.write_u16(1);
    writer.write_u16(0x0409);
    writer.write_u16(1);
    writer.write_u16(10);
    writer.write_u16(0);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let platform_id = reader.read_u16().unwrap();
    assert_eq!(platform_id, 3);
    
    let encoding_id = reader.read_u16().unwrap();
    assert_eq!(encoding_id, 1);
    
    let language_id = reader.read_u16().unwrap();
    assert_eq!(language_id, 0x0409);
    
    let name_id = reader.read_u16().unwrap();
    assert_eq!(name_id, 1);
    
    let length = reader.read_u16().unwrap();
    assert_eq!(length, 10);
    
    let offset = reader.read_u16().unwrap();
    assert_eq!(offset, 0);
}

#[test]
fn test_hmtx_long_metric() {
    let mut writer = FontWriter::new();
    
    writer.write_u16(500);
    writer.write_i16(50);
    writer.write_u16(600);
    writer.write_i16(60);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let advance_width_1 = reader.read_u16().unwrap();
    let lsb_1 = reader.read_i16().unwrap();
    assert_eq!(advance_width_1, 500);
    assert_eq!(lsb_1, 50);
    
    let advance_width_2 = reader.read_u16().unwrap();
    let lsb_2 = reader.read_i16().unwrap();
    assert_eq!(advance_width_2, 600);
    assert_eq!(lsb_2, 60);
}

#[test]
fn test_loca_short_format() {
    let mut writer = FontWriter::new();
    
    writer.write_u16(0);
    writer.write_u16(10);
    writer.write_u16(20);
    writer.write_u16(30);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let offset_0 = reader.read_u16().unwrap() as u32 * 2;
    let offset_1 = reader.read_u16().unwrap() as u32 * 2;
    let offset_2 = reader.read_u16().unwrap() as u32 * 2;
    
    assert_eq!(offset_0, 0);
    assert_eq!(offset_1, 20);
    assert_eq!(offset_2, 40);
}

#[test]
fn test_loca_long_format() {
    let mut writer = FontWriter::new();
    
    writer.write_u32(0);
    writer.write_u32(100);
    writer.write_u32(200);
    writer.write_u32(300);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let offset_0 = reader.read_u32().unwrap();
    let offset_1 = reader.read_u32().unwrap();
    let offset_2 = reader.read_u32().unwrap();
    
    assert_eq!(offset_0, 0);
    assert_eq!(offset_1, 100);
    assert_eq!(offset_2, 200);
}

#[test]
fn test_os2_table_parse() {
    let mut writer = FontWriter::new();
    
    writer.write_u16(4);
    writer.write_i16(500);
    writer.write_u16(400);
    writer.write_u16(5);
    writer.write_u16(0);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let version = reader.read_u16().unwrap();
    assert_eq!(version, 4);
    
    let x_avg_char_width = reader.read_i16().unwrap();
    assert_eq!(x_avg_char_width, 500);
    
    let us_weight_class = reader.read_u16().unwrap();
    assert_eq!(us_weight_class, 400);
    
    let us_width_class = reader.read_u16().unwrap();
    assert_eq!(us_width_class, 5);
}
