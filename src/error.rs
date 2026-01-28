use std::io;

pub type Result<T> = std::result::Result<T, TtfError>;

#[derive(Debug, thiserror::Error)]
pub enum TtfError {
    #[error("Invalid TTF signature: expected {expected:#x}, got {actual:#x}")]
    InvalidSignature { expected: u32, actual: u32 },

    #[error("Invalid table checksum for table: {0}")]
    InvalidChecksum(String),

    #[error("Required table not found: {0}")]
    MissingTable(String),

    #[error("Invalid table offset: {0}")]
    InvalidOffset(u64),

    #[error("Invalid table size: expected {expected}, got {actual}")]
    InvalidSize { expected: u64, actual: u64 },

    #[error("Unsupported table version: {0}")]
    UnsupportedVersion(u32),

    #[error("Invalid glyph index: {0}")]
    InvalidGlyphIndex(u16),

    #[error("Invalid number of glyphs: {0}")]
    InvalidNumGlyphs(u16),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(String),

    #[error("Invalid offset in loca table: {0}")]
    InvalidLocaOffset(u32),
}
