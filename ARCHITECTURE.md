# Architecture

## Design Principles

### DRY (Don't Repeat Yourself)
- Shared binary I/O utilities (`FontReader`, `FontWriter`)
- Common table parsing patterns via traits
- Reusable checksum calculation
- Single source of truth for table structures

### KISS (Keep It Simple, Stupid)
- Straightforward API design
- Minimal abstraction layers
- Direct mapping to TTF specification
- Clear error messages

### Separation of Concerns
- Binary I/O layer (`stream.rs`)
- Table parsing layer (`tables/`)
- High-level API layer (`font.rs`)
- Error handling layer (`error.rs`)

### Test-Friendly Design
- Pure functions where possible
- Dependency injection via traits
- Small, atomic functions
- Clear input/output contracts

## Module Structure

```
ttf-rs/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── error.rs            # Error types and Result alias
│   ├── font.rs             # Font struct with high-level API
│   ├── stream.rs           # Binary I/O utilities
│   └── tables/             # TTF table parsers
│       ├── mod.rs          # Table traits and common types
│       ├── head.rs         # Font header table
│       ├── maxp.rs         # Maximum profile table
│       ├── cmap.rs         # Character to glyph mapping
│       ├── name.rs         # Naming table
│       ├── hhea.rs         # Horizontal header
│       ├── hmtx.rs         # Horizontal metrics
│       ├── glyf.rs         # Glyph data
│       ├── loca.rs         # Index to location
│       ├── post.rs         # PostScript information
│       └── os2.rs          # OS/2 and Windows metrics
└── examples/               # Usage examples
    ├── basic.rs            # Basic font loading
    ├── save_font.rs        # Font serialization
    ├── glyph_info.rs       # Glyph data access
    └── font_metrics.rs     # Font metrics extraction
```

## Core Components

### 1. Binary I/O Layer (`stream.rs`)

**Purpose**: Handle big-endian binary data reading and writing

**Components**:
- `FontReader<'a>`: Zero-copy reader with position tracking
- `FontWriter`: Buffer-based writer for serialization
- `calculate_checksum()`: TTF checksum calculation

**Design Decisions**:
- Zero-copy reading via slice references
- Automatic big-endian conversion
- Position tracking for seeking
- Error propagation for EOF conditions

```rust
pub struct FontReader<'a> {
    data: &'a [u8],
    position: usize,
}

pub struct FontWriter {
    data: Vec<u8>,
}
```

### 2. Error Handling (`error.rs`)

**Purpose**: Comprehensive error types for all failure modes

**Design Decisions**:
- Use `thiserror` for derive macros
- Specific error variants for different failures
- Context-rich error messages
- `Result<T>` type alias for convenience

```rust
#[derive(Debug, thiserror::Error)]
pub enum FontError {
    InvalidFormat,
    TableNotFound(String),
    InvalidTableData(String),
    UnexpectedEof,
    Io(#[from] std::io::Error),
    InvalidChecksum,
    UnsupportedFormat(String),
}
```

### 3. Table Parsing Layer (`tables/`)

**Purpose**: Parse and serialize individual TTF tables

**Design Pattern**: Each table has:
- Struct representing parsed data
- `parse()` method for reading
- `write()` method for serialization
- Helper methods for common operations

**Common Traits**:
```rust
pub trait Table: Sized {
    fn parse(reader: &mut FontReader) -> Result<Self>;
    fn write(&self, writer: &mut FontWriter) -> Result<()>;
}
```

**Table Dependencies**:
- `hmtx` depends on `hhea` and `maxp` for counts
- `glyf` depends on `loca` for offsets
- `loca` depends on `head` for format
- Tables are parsed on-demand (lazy loading)

### 4. High-Level API (`font.rs`)

**Purpose**: Provide convenient font manipulation interface

**Design Decisions**:
- Store raw font data for zero-copy parsing
- Cache table records for fast lookup
- Lazy table parsing (parse on access)
- Convenience methods for common operations

```rust
pub struct Font {
    data: Vec<u8>,
    table_records: Vec<TableRecord>,
}

impl Font {
    // Loading
    pub fn load(path) -> Result<Self>;
    pub fn from_data(data) -> Result<Self>;
    
    // Table access
    pub fn head_table() -> Result<HeadTable>;
    pub fn cmap_table() -> Result<CmapTable>;
    // ... other tables
    
    // Convenience
    pub fn char_to_glyph(c) -> Result<u16>;
    pub fn num_glyphs() -> Result<u16>;
    
    // Writing
    pub fn save(path) -> Result<()>;
    pub fn to_bytes() -> Result<Vec<u8>>;
}
```

## Data Flow

### Reading a Font

```
File/Bytes
    ↓
Font::load() / Font::from_data()
    ↓
Parse SFNT header (scaler type, num tables)
    ↓
Parse table directory (tag, checksum, offset, length)
    ↓
Store raw data + table records
    ↓
Font struct (lazy parsing)
    ↓
User calls font.head_table()
    ↓
Find table record by tag
    ↓
Create FontReader at table offset
    ↓
HeadTable::parse(reader)
    ↓
Return parsed HeadTable
```

### Writing a Font

```
Font struct with raw data
    ↓
Font::to_bytes()
    ↓
Create FontWriter
    ↓
Write SFNT header
    ↓
Calculate table offsets
    ↓
Write table directory
    ↓
Write table data (from original raw data)
    ↓
Calculate checksums
    ↓
Return Vec<u8>
    ↓
Font::save() writes to file
```

### Character to Glyph Mapping

```
User calls font.char_to_glyph('A')
    ↓
Get cmap table
    ↓
Find best subtable (platform/encoding)
    ↓
Match format (0, 4, etc.)
    ↓
Format-specific lookup
    ↓
Return glyph index
```

## Memory Management

### Zero-Copy Strategy
- `FontReader` holds slice references (`&'a [u8]`)
- No data copying during parsing where possible
- Original font data stored in `Font` struct
- Tables parsed on-demand

### Ownership Model
- `Font` owns the raw font data (`Vec<u8>`)
- Table structs own their parsed data
- No shared ownership (no `Rc`/`Arc` needed)
- Clear lifetime boundaries

### Memory Efficiency
- Lazy table parsing (only parse what's needed)
- No caching of parsed tables (parse on each access)
- Small memory footprint for `Font` struct
- Future: Consider caching frequently accessed tables

## Extensibility

### Adding New Tables

1. Create new file in `tables/` (e.g., `kern.rs`)
2. Define table struct with parsed fields
3. Implement `parse()` method using `FontReader`
4. Implement `write()` method using `FontWriter`
5. Add accessor method in `Font` struct
6. Export from `tables/mod.rs`
7. Add tests

Example:
```rust
// tables/kern.rs
pub struct KernTable {
    pub version: u16,
    pub pairs: Vec<KernPair>,
}

impl KernTable {
    pub fn parse(reader: &mut FontReader) -> Result<Self> {
        // Parse implementation
    }
    
    pub fn write(&self, writer: &mut FontWriter) -> Result<()> {
        // Write implementation
    }
}

// font.rs
impl Font {
    pub fn kern_table(&self) -> Result<KernTable> {
        let data = self.get_table_data(b"kern")?;
        let mut reader = FontReader::new(&data);
        KernTable::parse(&mut reader)
    }
}
```

### Adding Font Modification

Future API design:
```rust
impl Font {
    pub fn modify(&mut self) -> FontModifier;
}

pub struct FontModifier<'a> {
    font: &'a mut Font,
}

impl<'a> FontModifier<'a> {
    pub fn set_font_name(&mut self, name: &str) -> Result<()>;
    pub fn set_version(&mut self, version: f64) -> Result<()>;
    pub fn update_metrics(&mut self, metrics: Metrics) -> Result<()>;
    pub fn commit(self) -> Result<()>;
}
```

## Testing Strategy

### Unit Tests
- Test each table parser independently
- Test binary I/O operations
- Test checksum calculation
- Test error conditions

### Integration Tests
- Load real font files
- Round-trip save/load
- Verify data integrity
- Test character mapping
- Test glyph data extraction

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_head_table() {
        // Minimal valid head table data
        let data = [...];
        let mut reader = FontReader::new(&data);
        let head = HeadTable::parse(&mut reader).unwrap();
        assert_eq!(head.units_per_em, 1000);
    }
}
```

## Performance Considerations

### Current Optimizations
- Zero-copy parsing where possible
- Lazy table loading
- Direct binary I/O (no intermediate allocations)
- Efficient big-endian conversion

### Future Optimizations
- Memory-mapped file support for large fonts
- Parallel table parsing
- Cache frequently accessed tables
- SIMD for checksum calculation
- Optimize cmap format 4 binary search

### Benchmarking
- Load time for various font sizes
- Table parsing performance
- Character lookup performance
- Serialization performance
- Memory usage profiling

## Error Handling Strategy

### Error Propagation
- Use `?` operator for error propagation
- Provide context in error messages
- Include table names in errors
- Include position information where relevant

### Error Recovery
- No partial parsing (all-or-nothing)
- Clear error messages for debugging
- Validation at parse time
- No silent failures

### Error Categories
1. **Format Errors**: Invalid font structure
2. **Data Errors**: Corrupted or invalid table data
3. **IO Errors**: File system issues
4. **Unsupported Errors**: Features not yet implemented

## Dependencies

### Current Dependencies
- `thiserror = "2.0"`: Error derive macros
- `hex = "0.4"` (dev): Testing and debugging

### Dependency Policy
- Minimize external dependencies
- Use standard library where possible
- Only add dependencies for significant value
- Avoid dependencies with large dependency trees
- Keep build times fast

## Future Architecture Enhancements

### Font Modification
- Builder pattern for font creation
- Transaction-based modifications
- Automatic table updates (e.g., update checksums)
- Validation before serialization

### Subsetting
- Glyph selection API
- Character range subsetting
- Automatic table pruning
- Cmap remapping

### Rendering
- Outline to path conversion
- Bezier curve evaluation
- Rasterization engine
- Hinting support

### Variable Fonts
- Variation axis support
- Instance generation
- Interpolation engine
- Delta processing

## Code Style Guidelines

### Naming Conventions
- Structs: `PascalCase` (e.g., `HeadTable`)
- Functions: `snake_case` (e.g., `char_to_glyph`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAGIC_NUMBER`)
- Type aliases: `PascalCase` (e.g., `Result`)

### Documentation
- Public API must have doc comments
- Include examples in doc comments
- Document error conditions
- Reference TTF spec where relevant

### Error Messages
- Be specific and actionable
- Include context (table name, position)
- Suggest fixes where possible
- Use proper grammar and punctuation

### Code Organization
- One table per file
- Group related functionality
- Keep functions small (<50 lines)
- Minimize public API surface
- Use `pub(crate)` for internal APIs

## Security Considerations

### Input Validation
- Validate all table sizes
- Check for integer overflow
- Validate offsets and lengths
- Prevent out-of-bounds access
- Validate checksums (optional)

### Memory Safety
- Use safe Rust (no unsafe unless necessary)
- Bounds checking on all array access
- Validate slice lengths
- Handle malformed fonts gracefully

### Denial of Service
- Limit recursion depth (composite glyphs)
- Validate table counts
- Check for circular references
- Timeout for long operations (future)

## References

- [Microsoft OpenType Specification](https://docs.microsoft.com/en-us/typography/opentype/spec/)
- [Apple TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/)
- [FreeType Documentation](https://freetype.org/freetype2/docs/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
