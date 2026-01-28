# ttf-rs

A comprehensive Rust library for reading, writing, and operating on TTF (TrueType Font) files.

## Why ttf-rs?

**Pure Rust Implementation**: Unlike many font libraries that rely on C bindings, ttf-rs is written entirely in Rust, giving you memory safety, thread safety, and easy cross-compilation without external dependencies.

**Read AND Write**: Most TTF parsers only read fonts. ttf-rs supports both reading and writing, enabling you to modify font files, create custom fonts, and build font tooling entirely in Rust.

**Production Ready**: With comprehensive table support (head, maxp, cmap, name, hhea, hmtx, glyf, loca, post, OS/2), robust error handling, and full test coverage, ttf-rs is suitable for production applications like font converters, web font optimizers, and custom text renderers.

**Flexible**: Whether you're building a font inspector, a subset generator for web fonts, a custom text layout engine, or font modification tools, ttf-rs provides the building blocks you need.

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

### Basic Font Loading

```rust
use ttf_rs::Font;

// Load a font from a file
let font = Font::load("path/to/font.ttf")?;

// Get basic font metrics
let head = font.head_table()?;
println!("Font Revision: {}", head.font_revision);
println!("Units per EM: {}", head.units_per_em);

// Get glyph count
let maxp = font.maxp_table()?;
println!("Total glyphs: {}", maxp.num_glyphs);
```

### Character to Glyph Mapping

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// Map a character to its glyph index
let glyph_index = font.char_to_glyph('A')?;
println!("'A' is glyph index {}", glyph_index);

// Get horizontal metrics for a glyph
let hmtx = font.hmtx_table()?;
let (advance, lsb) = hmtx.get_horizontal_metrics(glyph_index);
println!("Advance width: {}, LSB: {}", advance, lsb);
```

### Inspecting Glyphs

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;
let glyf = font.glyf_table()?;

// Get a glyph by index
if let Some(glyph) = glyf.get_glyph(glyph_index) {
    match &glyph.data {
        ttf_rs::tables::glyf::GlyphData::Simple(simple) => {
            println!("Simple glyph with {} contours and {} points",
                     glyph.number_of_contours,
                     simple.x_coordinates.len());
        }
        ttf_rs::tables::glyf::GlyphData::Composite(composite) => {
            println!("Composite glyph with {} components",
                     composite.components.len());
        }
        ttf_rs::tables::glyf::GlyphData::Empty => {
            println!("Empty glyph (whitespace)");
        }
    }
}
```

### Font Metrics for Text Layout

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// Get vertical metrics for line spacing
let hhea = font.hhea_table()?;
let line_gap = hhea.line_gap;
let line_height = hhea.get_line_height();
println!("Line height: {} (ascent: {} + descent: {} + gap: {})",
         line_height, hhea.ascent, hhea.descent, line_gap);

// Get OS/2 metrics for more precise spacing
if let Some(os2) = font.os2_table() {
    println!("Typo Ascender: {}", os2.typo_ascender);
    println!("Typo Descender: {}", os2.typo_descender);
    println!("X-Height: {}", os2.x_height);
    println!("Cap Height: {}", os2.cap_height);
}
```

### Saving Modified Fonts

```rust
use ttf_rs::Font;

let font = Font::load("font.ttf")?;

// After any modifications, save the font
font.save("output.ttf")?;

// Or get the raw bytes
let bytes = font.to_bytes()?;
std::fs::write("output.ttf", bytes)?;
```

## Examples

The library includes comprehensive examples demonstrating real-world usage:

```bash
# Basic font information and inspection
cargo run --example basic

# Character to glyph mapping demonstration
cargo run --example character_mapping

# Detailed glyph information (simple and composite)
cargo run --example glyph_info

# Glyph metrics for text layout
cargo run --example glyph_metrics

# Comprehensive font metrics display
cargo run --example font_metrics

# Table inspector - list all font tables
cargo run --example table_inspector

# Save and round-trip a font file
cargo run --example save_font

# Modify font properties
cargo run --example modify_font

# Complete feature demonstration
cargo run --example comprehensive

# Interactive demo
cargo run --example demo
```

### Example: Font Metrics Calculator

```rust
use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = Font::load("font.ttf")?;

    // Get essential metrics for text layout
    let hhea = font.hhea_table()?;
    let head = font.head_table()?;
    let os2 = font.os2_table()?;

    println!("Font Metrics for Text Layout:");
    println!("  Ascent: {}", hhea.ascent);
    println!("  Descent: {}", hhea.descent);
    println!("  Line Gap: {}", hhea.line_gap);
    println!("  Line Height: {}", hhea.get_line_height());
    println!("  Units per EM: {}", head.units_per_em);

    if let Some(os2) = os2 {
        println!("  X-Height: {}", os2.x_height);
        println!("  Cap Height: {}", os2.cap_height);
    }

    Ok(())
}
```

### Example: Character Mapping

```rust
use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = Font::load("font.ttf")?;
    let text = "Hello";

    println!("Mapping characters to glyphs:");
    for ch in text.chars() {
        if let Ok(glyph_index) = font.char_to_glyph(ch) {
            println!("  '{}' -> glyph {}", ch, glyph_index);
        }
    }

    Ok(())
}
```

### Example: Font Inspector

```rust
use ttf_rs::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = Font::load("font.ttf")?;

    println!("Font Tables:");
    for table in font.list_tables() {
        println!("  - {}", table);
    }

    println!("\nGlyph Count: {}", font.maxp_table()?.num_glyphs);

    if let Some(name) = font.name_table() {
        if let Some(family) = name.get_font_family() {
            println!("Font Family: {}", family);
        }
    }

    Ok(())
}
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
