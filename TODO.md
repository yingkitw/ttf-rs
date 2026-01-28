# TODO

## High Priority

### ✅ Core Infrastructure
- [x] Binary I/O utilities (FontReader, FontWriter)
- [x] Error handling with thiserror
- [x] Basic font loading and parsing
- [x] Table directory parsing
- [x] Checksum calculation

### ✅ Essential Tables
- [x] head - Font header
- [x] maxp - Maximum profile
- [x] cmap - Character mapping (formats 0, 4)
- [x] name - Naming table
- [x] hhea - Horizontal header
- [x] hmtx - Horizontal metrics
- [x] glyf - Glyph data (simple and composite)
- [x] loca - Index to location
- [x] post - PostScript information
- [x] OS/2 - OS/2 and Windows metrics

### ✅ Font Writing
- [x] Font serialization to bytes
- [x] Save fonts to disk
- [x] Table checksum calculation
- [x] SFNT header reconstruction

### ✅ Testing & Quality
- [x] Unit tests for all table parsers
- [x] Integration tests with real fonts (via examples)
- [x] Round-trip save/load tests
- [x] Character mapping tests
- [x] Glyph data extraction tests
- [x] Error handling tests
- [x] Binary I/O tests with edge cases

### ✅ Documentation
- [x] SPEC.md - Technical specifications
- [x] README.md - User documentation
- [x] TODO.md - Task tracking
- [x] ARCHITECTURE.md - Design documentation
- [ ] Inline code documentation improvements
- [ ] More usage examples

## Medium Priority

### Font Modification API
- [x] Modify font metadata (name table entries)
- [x] Set font name (family name)
- [x] Set full font name
- [x] Set version string
- [x] Set copyright notice
- [x] Set trademark
- [ ] Update font metrics (head, hhea, OS/2)
- [ ] Modify glyph advance widths
- [ ] Update font version and revision in head table
- [ ] Modify embedding permissions
- [ ] Add/update font names in multiple languages
- [ ] Serialize modified tables back to font

### Additional cmap Formats
- [ ] Format 6 - Trimmed table mapping
- [ ] Format 12 - Segmented coverage (full Unicode)
- [ ] Format 13 - Many-to-one range mappings
- [ ] Format 14 - Unicode variation sequences

### Advanced Glyph Operations
- [ ] Glyph bounding box calculation
- [ ] Glyph outline transformation (scale, rotate, translate)
- [ ] Glyph outline simplification
- [ ] Composite glyph resolution

### Font Subsetting
- [ ] Create subset with specified glyphs
- [ ] Remap character codes
- [ ] Remove unused tables
- [ ] Optimize subset size
- [ ] Preserve font metrics

## Low Priority

### Variable Fonts Support
- [ ] fvar - Font variations table
- [ ] gvar - Glyph variations table
- [ ] avar - Axis variations table
- [ ] STAT - Style attributes table
- [ ] HVAR/VVAR - Metric variations

### TrueType Instructions
- [ ] fpgm - Font program
- [ ] prep - Control value program
- [ ] cvt - Control value table
- [ ] Instruction execution engine (optional)

### Color Fonts
- [ ] COLR - Color table
- [ ] CPAL - Color palette table
- [ ] SVG - SVG table
- [ ] CBDT/CBLC - Color bitmap data
- [ ] sbix - Standard bitmap graphics

### Rendering & Rasterization
- [ ] Outline to path conversion
- [ ] Bezier curve evaluation
- [ ] Rasterization to bitmap
- [ ] Anti-aliasing support
- [ ] Hinting support

### Additional Tables
- [ ] kern - Kerning table
- [ ] GPOS - Glyph positioning
- [ ] GSUB - Glyph substitution
- [ ] BASE - Baseline data
- [ ] JSTF - Justification data

### Font Validation
- [ ] Validate table checksums
- [ ] Validate table structure
- [ ] Check required tables present
- [ ] Validate glyph data integrity
- [ ] Validate cmap consistency
- [ ] Report validation errors

### Performance Optimization
- [ ] Benchmark suite
- [ ] Lazy table loading
- [ ] Memory-mapped file support
- [ ] Parallel table parsing
- [ ] Cache frequently accessed data
- [ ] Optimize binary I/O

### Format Conversion
- [ ] TTF to OTF conversion
- [ ] OTF to TTF conversion
- [ ] WOFF/WOFF2 support
- [ ] EOT support

### Developer Tools
- [ ] CLI tool for font inspection
- [ ] Font diff tool
- [ ] Font validation tool
- [ ] Font subsetting CLI
- [ ] Font metrics reporter

## Completed

### Phase 1: Core Library ✅
- [x] Project structure setup
- [x] Binary I/O utilities
- [x] Error handling
- [x] Font loading from file/bytes
- [x] Table directory parsing
- [x] All essential table parsers
- [x] Character to glyph mapping
- [x] Font serialization and saving
- [x] Basic examples (4 examples)
- [x] README documentation
- [x] Updated to Rust edition 2024
- [x] Comprehensive test suite (37 tests passing)
- [x] Font modification API (basic implementation)

## Notes

### Testing Strategy
- Use real font files for integration tests
- Create minimal test fonts for unit tests
- Test edge cases (empty glyphs, large fonts, etc.)
- Verify round-trip save/load integrity
- Test error conditions

### Code Quality
- Follow Rust 2024 edition best practices
- Maintain DRY principles
- Keep functions small and testable
- Minimize public API surface
- Use traits for extensibility
- Comprehensive error messages

### Performance Targets
- Load 1MB font: <10ms
- Parse all tables: <5ms
- Character lookup: <1μs
- Glyph data access: <10μs
- Save font: <20ms

### Dependencies Policy
- Minimize external dependencies
- Use standard library where possible
- Only add dependencies for significant value
- Keep build times fast
