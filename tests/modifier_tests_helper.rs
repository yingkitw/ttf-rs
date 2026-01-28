// Helper function to create minimal hhea table
use ttf_rs::FontWriter;

pub fn create_minimal_hhea_table() -> Vec<u8> {
    let mut writer = FontWriter::new();
    writer.write_fixed(1.0); // version
    writer.write_i16(800); // ascent
    writer.write_i16(-200); // descent
    writer.write_i16(100); // line gap
    writer.write_u16(1000); // advance width max
    writer.write_i16(-50); // min left side bearing
    writer.write_i16(-50); // min right side bearing
    writer.write_i16(1000); // x max extent
    writer.write_i16(1); // caret slope rise
    writer.write_i16(0); // caret slope run
    writer.write_i16(0); // caret offset
    writer.write_i16(0); // reserved
    writer.write_i16(0); // reserved
    writer.write_i16(0); // reserved
    writer.write_i16(0); // reserved
    writer.write_i16(0); // metric data format
    writer.write_u16(10); // number of h metrics
    writer.into_inner()
}
