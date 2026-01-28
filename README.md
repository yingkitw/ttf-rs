# ttf-rs

A comprehensive Rust library for reading, writing, and operating on TTF (TrueType Font) files.

## Features

### Reading TTF Files
- **Parse TTF files** - Read and extract information from TrueType font files
- **Comprehensive table support** - Support for essential TTF tables:
  - `head` - Font header
  - `maxp` - Maximum profile
  - `cmap` - Character to glyph mapping (formats 0 and 4)
  - `name` - Naming table
  - `hhea` - Horizontal header
  - `hmtx` - Horizontal metrics
  - `glyf` - Glyph data (simple and composite glyphs)
  - `loca` - Index to location
  - `post` - PostScript information
  - `OS/2` - OS/2 and Windows metrics
- **Character mapping** - Map characters to glyph indices
- **Font metrics** - Access font metrics like ascent, descent, line height
- **Glyph information** - Read glyph outline data (simple and composite glyphs)

### Writing TTF Files
- **Save fonts** - Write modified fonts back to disk
- **Font serialization** - Convert font structures to raw bytes
- **Table modification** - Modify and update font table data
- **Checksum calculation** - Proper TTF checksums for tables

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ttf-rs = "0.1.0"
```

## Usage

### Loading a Font

```rust
use ttf_rs::Font;

// Load a font from a file
let font = Font::load("path/to/font.ttf")?;

// Or load from bytes
let data = std::fs::read("path/to/font.ttf")?;
let font = Font::from_data(data)?;
```

### Reading Font Information

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// Get header information
let head = font.head_table()?;
println!("Units per EM: {}", head.units_per_em);
println!("Font Revision: {}", head.font_revision);

// Get maximum profile
let maxp = font.maxp_table()?;
println!("Number of glyphs: {}", maxp.num_glyphs);

// Get horizontal header
let hhea = font.hhea_table()?;
println!("Ascent: {}", hhea.ascent);
println!("Descent: {}", hhea.descent);
println!("Line Height: {}", hhea.get_line_height());
```

### Character to Glyph Mapping

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// Map a character to a glyph index
let glyph_index = font.char_to_glyph('A')?;
println!("Glyph index for 'A': {}", glyph_index);

// Or use the cmap table directly
let cmap = font.cmap_table()?;
if let Some(glyph) = cmap.map_char('A') {
    println!("Glyph index for 'A': {}", glyph);
}
```

### Listing Tables

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// List all tables in the font
for table in font.list_tables() {
    println!("Table: {}", table);
}
```

### Reading Glyph Data

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// Get the glyf table
let glyf = font.glyf_table()?;

// Access a specific glyph
if let Some(glyph) = glyf.get_glyph(0) {
    println!("Glyph has {} contours", glyph.number_of_contours);

    match &glyph.data {
        ttf_rs::tables::glyf::GlyphData::Simple(simple) => {
            println!("Simple glyph with {} points",
                     simple.x_coordinates.len());
        }
        ttf_rs::tables::glyf::GlyphData::Composite(composite) => {
            println!("Composite glyph with {} components",
                     composite.components.len());
        }
        ttf_rs::tables::glyf::GlyphData::Empty => {
            println!("Empty glyph");
        }
    }
}
```

### Saving a Font

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// Save to a new file
font.save("output.ttf")?;

// Or get raw bytes
let bytes = font.to_bytes()?;
std::fs::write("output.ttf", bytes)?;
```

### Binary Utilities

The library also provides low-level binary reading/writing utilities:

```rust
use ttf_rs::{FontReader, FontWriter, calculate_checksum};

// Reading binary data
let mut reader = FontReader::new(data);
let value = reader.read_u32()?;
let fixed = reader.read_fixed()?;

// Writing binary data
let mut writer = FontWriter::new();
writer.write_u32(0x10000);
writer.write_fixed(1.5);

// Calculate TTF checksums
let checksum = calculate_checksum(&table_data);
```

## Examples

The library includes several examples demonstrating different features:

```bash
# Basic font information
cargo run --example basic

# Save and round-trip a font
cargo run --example save_font

# Detailed glyph information
cargo run --example glyph_info

# Comprehensive font metrics
cargo run --example font_metrics
```

## Project Structure

```
src/
├── lib.rs          # Library entry point
├── error.rs        # Error types (using thiserror)
├── font.rs         # Main Font struct with high-level API
├── stream.rs       # Binary reading/writing utilities
└── tables/         # TTF table parsers
    ├── mod.rs      # Table traits and records
    ├── head.rs     # Font header
    ├── maxp.rs     # Maximum profile
    ├── cmap.rs     # Character mapping
    ├── name.rs     # Naming table
    ├── hhea.rs     # Horizontal header
    ├── hmtx.rs     # Horizontal metrics
    ├── glyf.rs     # Glyph data
    ├── loca.rs     # Index to location
    ├── post.rs     # PostScript info
    └── os2.rs      # OS/2 & Windows metrics
```

## Implementation Status

### Currently Supported

- ✅ Reading TTF files
- ✅ Writing TTF files
- ✅ Parsing all essential tables
- ✅ Character to glyph mapping (cmap formats 0, 4)
- ✅ Font metrics extraction
- ✅ Glyph outline data (simple and composite)
- ✅ Binary data reading/writing utilities
- ✅ Table checksum calculation
- ✅ Font serialization and saving

### Planned Features

- ⏳ Font modification API
- ⏳ Subset font creation
- ⏳ Glyph outline rendering
- ⏳ Additional cmap formats (6, 12)
- ⏳ Variable fonts (gvar, fvar, avar tables)
- ⏳ TrueType instructions (fpgm, prep, cvt)
- ⏳ Color fonts (COLR, CPAL tables)

## API Design

The library follows these principles:

1. **Safe by default** - Uses Rust's type system for memory safety
2. **Zero-copy where possible** - References into original data when feasible
3. **Explicit error handling** - Uses `Result` types for all fallible operations
4. **Big-endian aware** - Automatically handles TTF's big-endian byte order
5. **Flexible API** - Both high-level convenience methods and low-level access

## Testing

Run the test suite:

```bash
cargo test
```

Build and check all examples:

```bash
cargo build --examples
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Areas of particular interest:

- Additional table formats
- Font modification utilities
- Subset creation
- Benchmarking and optimization
- Documentation improvements

## References

- [Microsoft OpenType Specification](https://docs.microsoft.com/en-us/typography/opentype/spec/)
- [Apple TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/)
- [FreeType Implementation Notes](https://freetype.org/freetype2/docs/reference/)

## Acknowledgments

This library was inspired by:
- [freetype](https://freetype.org/) - The gold standard for font rendering
- [fonttools](https://github.com/fonttools/fonttools) - Python font library
- [ttf-parser](https://github.com/RazrFalcon/ttf-parser) - Another Rust TTF parser
