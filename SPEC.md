# ttf-rs Technical Specification

## Overview

`ttf-rs` is a comprehensive Rust library for reading, writing, and manipulating TrueType Font (TTF) files. The library provides both high-level convenience APIs and low-level access to font structures.

## Technical Requirements

### Language & Edition
- **Rust Edition**: 2024
- **Minimum Rust Version**: 1.80.0+
- **Dependencies**: Minimal external dependencies (thiserror for error handling)

### Performance Goals
- Zero-copy parsing where possible
- Lazy loading of font tables
- Efficient binary I/O operations
- Memory-efficient glyph data access

### Safety & Correctness
- Memory-safe operations (no unsafe code unless absolutely necessary)
- Proper error handling for all fallible operations
- Big-endian byte order handling
- Checksum validation for font integrity

## Font File Format

### SFNT Structure
TTF files follow the SFNT (Scalable Font) format:

```
Offset Table (12 bytes)
├── scaler_type: u32      (0x00010000 for TrueType)
├── num_tables: u16       (number of tables)
├── search_range: u16     (largest power of 2 <= num_tables * 16)
├── entry_selector: u16   (log2(search_range/16))
└── range_shift: u16      (num_tables * 16 - search_range)

Table Records (16 bytes each)
├── tag: [u8; 4]          (table identifier)
├── checksum: u32         (table checksum)
├── offset: u32           (offset from beginning of file)
└── length: u32           (length of table in bytes)

Table Data
└── [variable length data for each table]
```

## Supported TTF Tables

### Required Tables

#### 1. head - Font Header
- **Size**: 54 bytes
- **Purpose**: Basic font-wide information
- **Key Fields**:
  - `version`: Fixed (1.0)
  - `font_revision`: Fixed
  - `checksum_adjustment`: u32
  - `magic_number`: u32 (0x5F0F3CF5)
  - `flags`: u16
  - `units_per_em`: u16 (16-2048, power of 2 recommended)
  - `created`: i64 (seconds since 1904-01-01)
  - `modified`: i64
  - `x_min`, `y_min`, `x_max`, `y_max`: i16
  - `mac_style`: u16
  - `lowest_rec_ppem`: u16
  - `font_direction_hint`: i16
  - `index_to_loc_format`: i16 (0=short, 1=long)
  - `glyph_data_format`: i16

#### 2. maxp - Maximum Profile
- **Size**: 32 bytes (version 1.0)
- **Purpose**: Memory allocation requirements
- **Key Fields**:
  - `version`: Fixed
  - `num_glyphs`: u16
  - `max_points`: u16
  - `max_contours`: u16
  - `max_composite_points`: u16
  - `max_composite_contours`: u16
  - `max_zones`: u16
  - `max_twilight_points`: u16
  - `max_storage`: u16
  - `max_function_defs`: u16
  - `max_instruction_defs`: u16
  - `max_stack_elements`: u16
  - `max_size_of_instructions`: u16
  - `max_component_elements`: u16
  - `max_component_depth`: u16

#### 3. cmap - Character to Glyph Mapping
- **Purpose**: Map Unicode code points to glyph indices
- **Supported Formats**:
  - **Format 0**: Byte encoding (256 glyphs max)
  - **Format 4**: Segment mapping (BMP only, most common)
  - **Format 6**: Trimmed table (planned)
  - **Format 12**: Segmented coverage (planned)

#### 4. name - Naming Table
- **Purpose**: Human-readable font names and metadata
- **Name IDs**:
  - 0: Copyright
  - 1: Font Family
  - 2: Font Subfamily
  - 3: Unique Identifier
  - 4: Full Font Name
  - 5: Version String
  - 6: PostScript Name
  - 7: Trademark
  - 8: Manufacturer
  - 9: Designer
  - 10: Description
  - 11: Vendor URL
  - 12: Designer URL

#### 5. hhea - Horizontal Header
- **Size**: 36 bytes
- **Purpose**: Horizontal layout metrics
- **Key Fields**:
  - `version`: Fixed
  - `ascent`: i16
  - `descent`: i16
  - `line_gap`: i16
  - `advance_width_max`: u16
  - `min_left_side_bearing`: i16
  - `min_right_side_bearing`: i16
  - `x_max_extent`: i16
  - `caret_slope_rise`: i16
  - `caret_slope_run`: i16
  - `caret_offset`: i16
  - `metric_data_format`: i16
  - `num_h_metrics`: u16

#### 6. hmtx - Horizontal Metrics
- **Purpose**: Per-glyph horizontal metrics
- **Structure**:
  - Long horizontal metrics: (advance_width: u16, lsb: i16) × num_h_metrics
  - Left side bearings: i16 × (num_glyphs - num_h_metrics)

#### 7. loca - Index to Location
- **Purpose**: Glyph data offsets in glyf table
- **Formats**:
  - Short (offset/2): u16 array
  - Long: u32 array

#### 8. glyf - Glyph Data
- **Purpose**: Glyph outline definitions
- **Glyph Types**:
  - **Simple Glyphs**: Contour-based outlines
    - On-curve points (bit 0 = 1)
    - Off-curve control points (bit 0 = 0)
  - **Composite Glyphs**: References to other glyphs with transformations
  - **Empty Glyphs**: No outline data

#### 9. post - PostScript Information
- **Purpose**: PostScript compatibility data
- **Key Fields**:
  - `version`: Fixed
  - `italic_angle`: Fixed
  - `underline_position`: i16
  - `underline_thickness`: i16
  - `is_fixed_pitch`: u32
  - Memory usage minimums

#### 10. OS/2 - OS/2 and Windows Metrics
- **Purpose**: OS/2 and Windows-specific metrics
- **Key Fields**:
  - `version`: u16
  - `x_avg_char_width`: i16
  - `us_weight_class`: u16
  - `us_width_class`: u16
  - `fs_type`: u16 (embedding permissions)
  - `y_subscript/superscript` metrics
  - `y_strikeout` metrics
  - `s_family_class`: i16
  - `panose`: [u8; 10]
  - Unicode ranges
  - `ach_vend_id`: [u8; 4]
  - `fs_selection`: u16
  - `us_first_char_index`: u16
  - `us_last_char_index`: u16
  - Typographic metrics

## API Specification

### High-Level API

#### Font Loading
```rust
pub struct Font {
    data: Vec<u8>,
    table_records: Vec<TableRecord>,
}

impl Font {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>;
    pub fn from_data(data: Vec<u8>) -> Result<Self>;
}
```

#### Table Access
```rust
impl Font {
    pub fn head_table(&self) -> Result<HeadTable>;
    pub fn maxp_table(&self) -> Result<MaxpTable>;
    pub fn cmap_table(&self) -> Result<CmapTable>;
    pub fn name_table(&self) -> Result<NameTable>;
    pub fn hhea_table(&self) -> Result<HheaTable>;
    pub fn hmtx_table(&self) -> Result<HmtxTable>;
    pub fn glyf_table(&self) -> Result<GlyfTable>;
    pub fn loca_table(&self) -> Result<LocaTable>;
    pub fn post_table(&self) -> Result<PostTable>;
    pub fn os2_table(&self) -> Result<Os2Table>;
}
```

#### Convenience Methods
```rust
impl Font {
    pub fn char_to_glyph(&self, c: char) -> Result<u16>;
    pub fn num_glyphs(&self) -> Result<u16>;
    pub fn units_per_em(&self) -> Result<u16>;
    pub fn list_tables(&self) -> Vec<String>;
}
```

#### Font Writing
```rust
impl Font {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    pub fn to_bytes(&self) -> Result<Vec<u8>>;
}
```

### Low-Level API

#### Binary Reading
```rust
pub struct FontReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> FontReader<'a> {
    pub fn new(data: &'a [u8]) -> Self;
    pub fn read_u8(&mut self) -> Result<u8>;
    pub fn read_i8(&mut self) -> Result<i8>;
    pub fn read_u16(&mut self) -> Result<u16>;
    pub fn read_i16(&mut self) -> Result<i16>;
    pub fn read_u32(&mut self) -> Result<u32>;
    pub fn read_i32(&mut self) -> Result<i32>;
    pub fn read_fixed(&mut self) -> Result<f64>;
    pub fn read_bytes(&mut self, count: usize) -> Result<&'a [u8]>;
    pub fn seek(&mut self, position: usize);
    pub fn position(&self) -> usize;
    pub fn remaining(&self) -> usize;
}
```

#### Binary Writing
```rust
pub struct FontWriter {
    data: Vec<u8>,
}

impl FontWriter {
    pub fn new() -> Self;
    pub fn write_u8(&mut self, value: u8);
    pub fn write_i8(&mut self, value: i8);
    pub fn write_u16(&mut self, value: u16);
    pub fn write_i16(&mut self, value: i16);
    pub fn write_u32(&mut self, value: u32);
    pub fn write_i32(&mut self, value: i32);
    pub fn write_fixed(&mut self, value: f64);
    pub fn write_bytes(&mut self, bytes: &[u8]);
    pub fn into_vec(self) -> Vec<u8>;
}
```

#### Checksum Calculation
```rust
pub fn calculate_checksum(data: &[u8]) -> u32;
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum FontError {
    #[error("Invalid font format")]
    InvalidFormat,
    
    #[error("Table not found: {0}")]
    TableNotFound(String),
    
    #[error("Invalid table data: {0}")]
    InvalidTableData(String),
    
    #[error("Unexpected end of data")]
    UnexpectedEof,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid checksum")]
    InvalidChecksum,
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

pub type Result<T> = std::result::Result<T, FontError>;
```

## Testing Requirements

### Unit Tests
- Binary I/O operations
- Checksum calculation
- Table parsing
- Error handling

### Integration Tests
- Load real TTF files
- Round-trip save/load
- Character to glyph mapping
- Glyph data extraction

### Test Coverage Goals
- Core functionality: >90%
- Table parsers: >85%
- Binary I/O: >95%

## Future Enhancements

### Phase 1: Font Modification
- Modify font metadata (name table)
- Update font metrics
- Add/remove glyphs
- Modify glyph outlines

### Phase 2: Advanced Features
- Font subsetting
- Glyph outline rendering
- Additional cmap formats (6, 12)
- Variable fonts support (gvar, fvar, avar)

### Phase 3: Extended Support
- TrueType instructions (fpgm, prep, cvt)
- Color fonts (COLR, CPAL, SVG)
- Font validation
- Format conversion

## Performance Benchmarks

Target performance metrics:
- Load 1MB font: <10ms
- Parse all tables: <5ms
- Character lookup: <1μs
- Glyph data access: <10μs
- Save font: <20ms

## Compatibility

### Font Format Support
- TrueType (.ttf)
- OpenType with TrueType outlines (.otf)

### Platform Support
- Linux
- macOS
- Windows
- WebAssembly (future)

## References

- [Microsoft OpenType Specification](https://docs.microsoft.com/en-us/typography/opentype/spec/)
- [Apple TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/)
- [ISO/IEC 14496-22:2019 (Open Font Format)](https://www.iso.org/standard/74461.html)
