use crate::error::Result;
use crate::font::Font;
use crate::tables::TtfTable;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Cached font wrapper with lazy table loading
#[derive(Clone)]
pub struct CachedFont {
    font: Arc<RwLock<Font>>,
    table_cache: Arc<RwLock<HashMap<Vec<u8>, Option<Vec<u8>>>>>,
}

impl CachedFont {
    pub fn new(font: Font) -> Self {
        Self {
            font: Arc::new(RwLock::new(font)),
            table_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load a font with caching enabled
    pub fn load_with_cache<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let font = Font::load(path)?;
        Ok(Self::new(font))
    }

    /// Get table data with caching
    pub fn get_table_data_cached(&self, tag: &[u8; 4]) -> Result<Option<Vec<u8>>> {
        // Check cache first
        {
            let cache = self.table_cache.read().unwrap();
            if let Some(data) = cache.get(&tag.to_vec()) {
                return Ok(data.clone());
            }
        }

        // Load table data
        let font = self.font.read().unwrap();
        let data = font.get_table_data(tag);

        // Update cache
        if data.is_some() {
            let mut cache = self.table_cache.write().unwrap();
            cache.insert(tag.to_vec(), data.clone());
        }

        Ok(data)
    }

    /// Get head table with caching
    pub fn head_table_cached(&self) -> Result<crate::tables::head::HeadTable> {
        if let Some(data) = self.get_table_data_cached(b"head")? {
            let mut reader = crate::stream::FontReader::from_slice(&data);
            crate::tables::head::HeadTable::from_reader(&mut reader, data.len() as u32)
        } else {
            let font = self.font.read().unwrap();
            font.head_table()
        }
    }

    /// Get maxp table with caching
    pub fn maxp_table_cached(&self) -> Result<crate::tables::maxp::MaxpTable> {
        if let Some(data) = self.get_table_data_cached(b"maxp")? {
            let mut reader = crate::stream::FontReader::from_slice(&data);
            crate::tables::maxp::MaxpTable::from_reader(&mut reader, data.len() as u32)
        } else {
            let font = self.font.read().unwrap();
            font.maxp_table()
        }
    }

    /// Get cmap table with caching
    pub fn cmap_table_cached(&self) -> Result<crate::tables::cmap::CmapTable> {
        if let Some(data) = self.get_table_data_cached(b"cmap")? {
            let mut reader = crate::stream::FontReader::from_slice(&data);
            crate::tables::cmap::CmapTable::from_reader(&mut reader, data.len() as u32)
        } else {
            let font = self.font.read().unwrap();
            font.cmap_table()
        }
    }

    /// Clear the table cache
    pub fn clear_cache(&self) {
        let mut cache = self.table_cache.write().unwrap();
        cache.clear();
    }

    /// Get the underlying font
    pub fn font(&self) -> Font {
        let font = self.font.read().unwrap();
        font.clone()
    }
}

impl Font {
    /// Create a cached wrapper for this font
    pub fn with_cache(self) -> CachedFont {
        CachedFont::new(self)
    }
}
