use ttf_rs::{FontReader, FontWriter, calculate_checksum};

#[test]
fn test_font_reader_u8() {
    let data = vec![0x12, 0x34, 0x56, 0x78];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_u8().unwrap(), 0x12);
    assert_eq!(reader.read_u8().unwrap(), 0x34);
    assert_eq!(reader.position(), 2);
}

#[test]
fn test_font_reader_i8() {
    let data = vec![0x7F, 0x80, 0xFF];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_i8().unwrap(), 127);
    assert_eq!(reader.read_i8().unwrap(), -128);
    assert_eq!(reader.read_i8().unwrap(), -1);
}

#[test]
fn test_font_reader_u16() {
    let data = vec![0x12, 0x34, 0x56, 0x78];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_u16().unwrap(), 0x1234);
    assert_eq!(reader.read_u16().unwrap(), 0x5678);
}

#[test]
fn test_font_reader_i16() {
    let data = vec![0x7F, 0xFF, 0x80, 0x00];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_i16().unwrap(), 32767);
    assert_eq!(reader.read_i16().unwrap(), -32768);
}

#[test]
fn test_font_reader_u32() {
    let data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_u32().unwrap(), 0x12345678);
    assert_eq!(reader.read_u32().unwrap(), 0x9ABCDEF0);
}

#[test]
fn test_font_reader_i32() {
    let data = vec![0x7F, 0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x00, 0x00];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_i32().unwrap(), 2147483647);
    assert_eq!(reader.read_i32().unwrap(), -2147483648);
}

#[test]
fn test_font_reader_fixed() {
    let data = vec![0x00, 0x01, 0x00, 0x00];
    let mut reader = FontReader::from_slice(&data);
    
    let fixed = reader.read_fixed().unwrap();
    assert!((fixed - 1.0).abs() < 0.0001);
}

#[test]
fn test_font_reader_position() {
    let data = vec![0x00; 10];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.position(), 0);
    reader.read_u16().unwrap();
    assert_eq!(reader.position(), 2);
    reader.read_u32().unwrap();
    assert_eq!(reader.position(), 6);
}

#[test]
fn test_font_reader_set_position() {
    let data = vec![0x00, 0x11, 0x22, 0x33, 0x44];
    let mut reader = FontReader::from_slice(&data);
    
    reader.set_position(2).unwrap();
    assert_eq!(reader.read_u8().unwrap(), 0x22);
    
    reader.set_position(0).unwrap();
    assert_eq!(reader.read_u8().unwrap(), 0x00);
}

#[test]
fn test_font_reader_skip() {
    let data = vec![0x00, 0x11, 0x22, 0x33, 0x44];
    let mut reader = FontReader::from_slice(&data);
    
    reader.skip(2).unwrap();
    assert_eq!(reader.read_u8().unwrap(), 0x22);
}

#[test]
fn test_font_reader_remaining() {
    let data = vec![0x00; 10];
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.remaining(), 10);
    reader.read_u32().unwrap();
    assert_eq!(reader.remaining(), 6);
}

#[test]
fn test_font_reader_eof() {
    let data = vec![0x12];
    let mut reader = FontReader::from_slice(&data);
    
    reader.read_u8().unwrap();
    assert!(reader.read_u8().is_err());
}

#[test]
fn test_font_writer_u8() {
    let mut writer = FontWriter::new();
    writer.write_u8(0x12);
    writer.write_u8(0x34);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x12, 0x34]);
}

#[test]
fn test_font_writer_i8() {
    let mut writer = FontWriter::new();
    writer.write_i8(127);
    writer.write_i8(-128);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x7F, 0x80]);
}

#[test]
fn test_font_writer_u16() {
    let mut writer = FontWriter::new();
    writer.write_u16(0x1234);
    writer.write_u16(0x5678);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn test_font_writer_i16() {
    let mut writer = FontWriter::new();
    writer.write_i16(32767);
    writer.write_i16(-32768);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x7F, 0xFF, 0x80, 0x00]);
}

#[test]
fn test_font_writer_u32() {
    let mut writer = FontWriter::new();
    writer.write_u32(0x12345678);
    writer.write_u32(0x9ABCDEF0);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
}

#[test]
fn test_font_writer_i32() {
    let mut writer = FontWriter::new();
    writer.write_i32(2147483647);
    writer.write_i32(-2147483648);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x7F, 0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x00, 0x00]);
}

#[test]
fn test_font_writer_fixed() {
    let mut writer = FontWriter::new();
    writer.write_fixed(1.0);
    writer.write_fixed(1.5);
    
    let data = writer.into_inner();
    assert_eq!(&data[0..4], &[0x00, 0x01, 0x00, 0x00]);
    assert_eq!(&data[4..8], &[0x00, 0x01, 0x80, 0x00]);
}

#[test]
fn test_font_writer_bytes() {
    let mut writer = FontWriter::new();
    writer.write_bytes(&[0x12, 0x34, 0x56, 0x78]);
    
    let data = writer.into_inner();
    assert_eq!(data, vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn test_calculate_checksum_aligned() {
    let data = vec![0x12, 0x34, 0x56, 0x78];
    let checksum = calculate_checksum(&data);
    assert_eq!(checksum, 0x12345678);
}

#[test]
fn test_calculate_checksum_unaligned() {
    let data = vec![0x12, 0x34, 0x56];
    let checksum = calculate_checksum(&data);
    assert_eq!(checksum, 0x12345600);
}

#[test]
fn test_calculate_checksum_empty() {
    let data = vec![];
    let checksum = calculate_checksum(&data);
    assert_eq!(checksum, 0);
}

#[test]
fn test_round_trip_u8() {
    let mut writer = FontWriter::new();
    writer.write_u8(0x42);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_u8().unwrap(), 0x42);
}

#[test]
fn test_round_trip_u16() {
    let mut writer = FontWriter::new();
    writer.write_u16(0x1234);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_u16().unwrap(), 0x1234);
}

#[test]
fn test_round_trip_u32() {
    let mut writer = FontWriter::new();
    writer.write_u32(0x12345678);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    assert_eq!(reader.read_u32().unwrap(), 0x12345678);
}

#[test]
fn test_round_trip_fixed() {
    let mut writer = FontWriter::new();
    writer.write_fixed(1.5);
    
    let data = writer.into_inner();
    let mut reader = FontReader::from_slice(&data);
    
    let value = reader.read_fixed().unwrap();
    assert!((value - 1.5).abs() < 0.0001);
}
