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
- [x] Inline code documentation improvements
- [x] More usage examples (8 examples total)

## Medium Priority

### Font Modification API
- [x] Modify font metadata (name table entries)
- [x] Set font name (family name)
- [x] Set full font name
- [x] Set version string
- [x] Set copyright notice
- [x] Set trademark
- [x] Update font metrics (head, hhea, OS/2)
- [x] Modify glyph advance widths
- [x] Update font version and revision in head table
- [x] Modify embedding permissions
- [x] Add/update font names in multiple languages
- [x] Basic table serialization (name, head, hhea, OS/2, hmtx)
- [ ] Full round-trip serialization with proper offset recalculation
- [ ] Complete font rebuilding after modifications

### Additional cmap Formats
- [x] Format 6 - Trimmed table mapping
- [x] Format 12 - Segmented coverage (full Unicode)
- [x] Format 13 - Many-to-one range mappings
- [x] Format 14 - Unicode variation sequences

### Advanced Glyph Operations
- [x] Glyph bounding box calculation
- [x] Glyph outline transformation (scale, rotate, translate)
- [x] Glyph outline simplification
- [x] Composite glyph resolution

### Font Subsetting
- [x] Basic subsetting infrastructure (FontSubset struct)
- [x] Add glyphs by ID
- [x] Add glyphs by character
- [x] Subset glyf table
- [x] Subset loca table
- [x] Subset hmtx table
- [x] Complete cmap table subsetting
- [ ] Remove unused tables
- [ ] Optimize subset size
- [ ] Full integration and testing

## Low Priority

### Variable Fonts Support
- [x] fvar - Font variations table
- [x] gvar - Glyph variations table
- [x] avar - Axis variations table
- [x] STAT - Style attributes table
- [x] HVAR/VVAR - Metric variations

### TrueType Instructions
- [x] fpgm - Font program
- [x] prep - Control value program
- [x] cvt - Control value table
- [ ] Instruction execution engine (optional)

### Color Fonts
- [x] COLR - Color table
- [x] CPAL - Color palette table
- [x] SVG - SVG table
- [x] CBDT/CBLC - Color bitmap data
- [x] sbix - Standard bitmap graphics

### Rendering & Rasterization
- [x] Outline to path conversion
- [x] Bezier curve evaluation
- [x] Rasterization to bitmap
- [x] Anti-aliasing support
- [ ] Hinting support

### Additional Tables
- [x] kern - Kerning table
- [x] GPOS - Glyph positioning
- [x] GSUB - Glyph substitution
- [x] BASE - Baseline data
- [x] JSTF - Justification data

### Font Validation
- [x] Validate table checksums
- [x] Validate table structure
- [x] Check required tables present
- [x] Validate glyph data integrity
- [x] Validate cmap consistency
- [x] Report validation errors

### Performance Optimization
- [ ] Benchmark suite
- [x] Lazy table loading
- [ ] Memory-mapped file support
- [ ] Parallel table parsing
- [x] Cache frequently accessed data
- [ ] Optimize binary I/O

### Format Conversion
- [ ] TTF to OTF conversion
- [ ] OTF to TTF conversion
- [x] WOFF/WOFF2 support
- [ ] EOT support

### Developer Tools
- [x] CLI tool for font inspection
- [ ] Font diff tool
- [x] Font validation tool
- [x] Font subsetting CLI
- [x] Font metrics reporter

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
- [x] Comprehensive test suite (43 tests passing)
- [x] Font modification API (comprehensive implementation)
- [x] Font subsetting infrastructure (foundation complete)
- [x] 8 working examples (basic, save_font, glyph_info, font_metrics, modify_font, character_mapping, table_inspector, glyph_metrics)
- [x] 4 CLI tools (ttf-info, ttf-validate, ttf-subset, ttf-metrics)
- [x] Comprehensive inline documentation for core modules

### Phase 2: Advanced Features ✅
- [x] Additional cmap formats (6, 12, 13, 14)
- [x] Advanced glyph operations (bounding box, transformation, simplification, composite resolution)
- [x] Font validation framework
- [x] Additional tables (kern, GPOS, GSUB, BASE, JSTF)
- [x] TrueType instruction tables (fpgm, prep, cvt)
- [x] Variable font support (fvar, gvar, avar, STAT, HVAR/VVAR)
- [x] Color font support (COLR, CPAL, SVG, CBDT/CBLC, sbix)
- [x] Rendering and rasterization (Rasterizer, RasterizedGlyph)
- [x] Format conversion (WOFF, WOFF2)
- [x] Performance optimization (CachedFont with lazy loading and caching)
- [x] Cmap table subsetting with Format 4 support

### Phase 3: Developer Tools ✅
- [x] Font metrics reporter (ttf-metrics CLI)
- [x] Enhanced subsetting with cmap support

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
