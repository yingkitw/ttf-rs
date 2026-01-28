use std::io;

/// Helper struct for reading TTF data with proper endianness handling
pub struct FontReader {
    data: Vec<u8>,
    pos: usize,
}

impl FontReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            data: slice.to_vec(),
            pos: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, pos: usize) -> Result<(), io::Error> {
        if pos > self.data.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Position out of bounds",
            ));
        }
        self.pos = pos;
        Ok(())
    }

    pub fn skip(&mut self, bytes: usize) -> Result<(), io::Error> {
        self.set_position(self.pos + bytes)
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }

    pub fn read_u8(&mut self) -> Result<u8, io::Error> {
        if self.pos + 1 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let val = self.data[self.pos];
        self.pos += 1;
        Ok(val)
    }

    pub fn read_i8(&mut self) -> Result<i8, io::Error> {
        self.read_u8().map(|b| b as i8)
    }

    pub fn read_u16(&mut self) -> Result<u16, io::Error> {
        if self.pos + 2 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let val = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(val)
    }

    pub fn read_i16(&mut self) -> Result<i16, io::Error> {
        self.read_u16().map(|b| b as i16)
    }

    pub fn read_u24(&mut self) -> Result<u32, io::Error> {
        if self.pos + 3 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let val = u32::from_be_bytes([
            0,
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
        ]);
        self.pos += 3;
        Ok(val)
    }

    pub fn read_u32(&mut self) -> Result<u32, io::Error> {
        if self.pos + 4 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let val = u32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(val)
    }

    pub fn read_i32(&mut self) -> Result<i32, io::Error> {
        self.read_u32().map(|b| b as i32)
    }

    pub fn read_u64(&mut self) -> Result<u64, io::Error> {
        if self.pos + 8 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let val = u64::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(val)
    }

    pub fn read_i64(&mut self) -> Result<i64, io::Error> {
        self.read_u64().map(|b| b as i64)
    }

    pub fn read_fixed(&mut self) -> Result<f32, io::Error> {
        self.read_i32().map(|i| i as f32 / 65536.0)
    }

    pub fn read_f2dot14(&mut self) -> Result<f32, io::Error> {
        self.read_i16().map(|i| i as f32 / 16384.0)
    }

    pub fn read_long_datetime(&mut self) -> Result<u64, io::Error> {
        // Long datetime is a 64-bit integer representing seconds since 1904
        self.read_u64()
    }

    pub fn read_tag(&mut self) -> Result<[u8; 4], io::Error> {
        if self.pos + 4 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let mut tag = [0u8; 4];
        tag.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(tag)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, io::Error> {
        if self.pos + len > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes"));
        }
        let bytes = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(bytes)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String, io::Error> {
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }
}

/// Helper struct for writing TTF data with proper endianness handling
pub struct FontWriter {
    data: Vec<u8>,
}

impl FontWriter {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn position(&self) -> usize {
        self.data.len()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn write_u8(&mut self, val: u8) {
        self.data.push(val);
    }

    pub fn write_i8(&mut self, val: i8) {
        self.data.push(val as u8);
    }

    pub fn write_u16(&mut self, val: u16) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_i16(&mut self, val: i16) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_u24(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_be_bytes()[1..]);
    }

    pub fn write_u32(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_i32(&mut self, val: i32) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_u64(&mut self, val: u64) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_i64(&mut self, val: i64) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    pub fn write_fixed(&mut self, val: f32) {
        let fixed = (val * 65536.0) as i32;
        self.write_i32(fixed);
    }

    pub fn write_f2dot14(&mut self, val: f32) {
        let f2dot14 = (val * 16384.0) as i16;
        self.write_i16(f2dot14);
    }

    pub fn write_long_datetime(&mut self, val: u64) {
        self.write_u64(val);
    }

    pub fn write_tag(&mut self, tag: &[u8; 4]) {
        self.data.extend_from_slice(tag);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn write_padding(&mut self, align: usize) {
        let pos = self.data.len();
        let padding = (align - (pos % align)) % align;
        self.data.extend(vec![0u8; padding]);
    }

    pub fn pad_to(&mut self, target_len: usize) {
        while self.data.len() < target_len {
            self.data.push(0);
        }
    }
}

impl Default for FontWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate TTF checksum
pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let mut len = data.len();

    // Handle data that's not a multiple of 4 bytes
    if len % 4 != 0 {
        len += 4 - (len % 4);
    }

    for i in (0..len).step_by(4) {
        let mut val: u32 = 0;
        for j in 0..4 {
            if i + j < data.len() {
                val = (val << 8) | data[i + j] as u32;
            } else {
                val <<= 8;
            }
        }
        sum = sum.wrapping_add(val);
    }

    sum
}
