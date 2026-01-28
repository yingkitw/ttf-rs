// ttf-rs: A Rust library for reading, writing, and operating on TTF files

mod error;
mod font;
mod tables;
mod stream;
mod modifier;
mod subset;
mod validation;

pub use error::{TtfError, Result};
pub use font::Font;
pub use modifier::FontModifier;
pub use subset::FontSubset;
pub use validation::{ValidationReport, ValidationError, ValidationWarning};
pub use stream::{FontReader, FontWriter, calculate_checksum};
pub use tables::{
    TableRecord,
    TtfTable,
    TtfTableWrite,
    head::HeadTable,
    maxp::MaxpTable,
    cmap::CmapTable,
    name::NameTable,
    hhea::HheaTable,
    hmtx::HmtxTable,
    glyf::{GlyfTable, GlyphData, Point, BoundingBox},
    loca::LocaTable,
    post::PostTable,
    os2::Os2Table,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_loads() {
        // Basic test to ensure library compiles
        assert!(true);
    }
}
