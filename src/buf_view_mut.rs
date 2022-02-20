use crate::macros::{buf_get_do, buf_read_do};
use std::io::{self, Write};

/// Wrap a &mut \[u8\] buffer as read and write.
///
/// BufViewMut support many methods to read/write primitive types from a byte buffer easily,
/// it support to read/write primitive types as big endian or little endian. BufViewMut wrap
/// the original buffer with reader_index and writer_index as fllowing structure.
/// When read data from it, the reader_index will advance the data length, and the
/// read must between the reader_index and writer_index. When write data to it,
/// the writer_index will advance the data length, too.
///
/// Any more, it support get method, too. It make random get data from the BufViewMut, and
/// support set method, too, which make random put data to the BufViewMut.
///
/// BufViewMut structure
/// ```text
/// -----------------------------------------------------
/// |       |                         |                 |
/// -----------------------------------------------------
///         ^                         ^                 ^
///         |                         |                 |
///   reader_index              writer_index        buf.len()
/// ```
/// Example
/// ```
/// use buf_view::BufViewMut;
///
/// let mut buf = [0u8;7];
/// let mut buf_view = BufViewMut::wrap(&mut buf);
///
/// buf_view.write_u8(0);
/// buf_view.write_u16(0x0102);
/// buf_view.write_u32_le(0x03040506);
///
/// assert_eq!(buf_view.read_u8(), 0);
/// assert_eq!(buf_view.read_u16(), 0x0102);
/// assert_eq!(buf_view.read_u32_le(), 0x03040506);
/// assert_eq!(buf_view.get_u16(1), 0x0102);
/// ```
///
#[derive(Debug)]
pub struct BufViewMut<'a> {
    buf: &'a mut [u8],
    reader_index: usize,
    writer_index: usize,
}

impl<'a> BufViewMut<'a> {
    /// Wrap the `buf` as BufViewMut, set the reader_index=0 and writer_index=0,
    /// this make the whole `buf` can write by default.
    pub fn wrap(buf: &'a mut [u8]) -> Self {
        BufViewMut::wrap_with(buf, 0, 0)
    }

    /// Wrap the `buf` as BufViewMut, and specify the reader_index and writer_index.
    /// ```
    /// use buf_view::BufViewMut;
    ///
    /// let mut buf = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let mut buf = BufViewMut::wrap_with(&mut buf, 1, 3);
    ///
    /// buf.write_u32(0x01020304);
    ///
    /// assert_eq!(buf.read_u16(), 0x0102);
    /// assert_eq!(buf.read_u32(), 0x01020304);
    /// ```
    pub fn wrap_with(buf: &'a mut [u8], reader_index: usize, writer_index: usize) -> Self {
        assert!(reader_index <= writer_index && buf.len() >= writer_index);
        BufViewMut {
            buf,
            reader_index,
            writer_index,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        assert!(self.remaining() >= 1);
        let val = self.buf[self.reader_index];
        self.reader_index += 1;
        val
    }

    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    pub fn read_u16(&mut self) -> u16 {
        buf_read_do!(self, u16, be);
    }

    pub fn read_u16_le(&mut self) -> u16 {
        buf_read_do!(self, u16, le);
    }

    pub fn read_i16(&mut self) -> i16 {
        buf_read_do!(self, i16, be);
    }

    pub fn read_i16_le(&mut self) -> i16 {
        buf_read_do!(self, i16, le);
    }

    pub fn read_u32(&mut self) -> u32 {
        buf_read_do!(self, u32, be);
    }

    pub fn read_u32_le(&mut self) -> u32 {
        buf_read_do!(self, u32, le);
    }

    pub fn read_i32(&mut self) -> i32 {
        buf_read_do!(self, i32, be);
    }

    pub fn read_i32_le(&mut self) -> i32 {
        buf_read_do!(self, i32, le);
    }

    pub fn read_u64(&mut self) -> u64 {
        buf_read_do!(self, u64, be);
    }

    pub fn read_u64_le(&mut self) -> u64 {
        buf_read_do!(self, u64, le);
    }

    pub fn read_i64(&mut self) -> i64 {
        buf_read_do!(self, i64, be);
    }

    pub fn read_i64_le(&mut self) -> i64 {
        buf_read_do!(self, i64, le);
    }

    pub fn read_u128(&mut self) -> u128 {
        buf_read_do!(self, u128, be);
    }

    pub fn read_u128_le(&mut self) -> u128 {
        buf_read_do!(self, u128, le);
    }

    pub fn read_i128(&mut self) -> i128 {
        buf_read_do!(self, i128, be);
    }

    pub fn read_i128_le(&mut self) -> i128 {
        buf_read_do!(self, i128, le);
    }

    pub fn read_f32(&mut self) -> f32 {
        buf_read_do!(self, f32, be);
    }

    pub fn read_f32_le(&mut self) -> f32 {
        buf_read_do!(self, f32, le);
    }

    pub fn read_f64(&mut self) -> f64 {
        buf_read_do!(self, f64, be);
    }

    pub fn read_f64_le(&mut self) -> f64 {
        buf_read_do!(self, f64, le);
    }

    pub fn read_bytes(&mut self, dest: &mut [u8]) -> usize {
        let left = self.remaining();
        assert!(left >= dest.len());
        let copy_len = if dest.len() < left { dest.len() } else { left };
        let end = self.reader_index + copy_len;
        dest[..copy_len].copy_from_slice(&self.buf[self.reader_index..end]);
        self.reader_index = end;
        copy_len
    }

    pub fn get_u8(&mut self, index: usize) -> u8 {
        assert!(self.buf.len() > index);
        self.buf[index]
    }

    pub fn get_i8(&mut self, index: usize) -> i8 {
        self.get_u8(index) as i8
    }

    pub fn get_u16(&mut self, index: usize) -> u16 {
        buf_get_do!(self, index, u16, be);
    }

    pub fn get_u16_le(&mut self, index: usize) -> u16 {
        buf_get_do!(self, index, u16, le);
    }

    pub fn get_i16(&mut self, index: usize) -> i16 {
        buf_get_do!(self, index, i16, be);
    }

    pub fn get_i16_le(&mut self, index: usize) -> i16 {
        buf_get_do!(self, index, i16, le);
    }

    pub fn get_u32(&mut self, index: usize) -> u32 {
        buf_get_do!(self, index, u32, be);
    }

    pub fn get_u32_le(&mut self, index: usize) -> u32 {
        buf_get_do!(self, index, u32, le);
    }

    pub fn get_i32(&mut self, index: usize) -> i32 {
        buf_get_do!(self, index, i32, be);
    }

    pub fn get_i32_le(&mut self, index: usize) -> i32 {
        buf_get_do!(self, index, i32, le);
    }

    pub fn get_u64(&mut self, index: usize) -> u64 {
        buf_get_do!(self, index, u64, be);
    }

    pub fn get_u64_le(&mut self, index: usize) -> u64 {
        buf_get_do!(self, index, u64, le);
    }

    pub fn get_i64(&mut self, index: usize) -> i64 {
        buf_get_do!(self, index, i64, be);
    }

    pub fn get_i64_le(&mut self, index: usize) -> i64 {
        buf_get_do!(self, index, i64, le);
    }

    pub fn get_u128(&mut self, index: usize) -> u128 {
        buf_get_do!(self, index, u128, be);
    }

    pub fn get_u128_le(&mut self, index: usize) -> u128 {
        buf_get_do!(self, index, u128, le);
    }

    pub fn get_i128(&mut self, index: usize) -> i128 {
        buf_get_do!(self, index, i128, be);
    }

    pub fn get_i128_le(&mut self, index: usize) -> i128 {
        buf_get_do!(self, index, i128, le);
    }

    pub fn get_f32(&mut self, index: usize) -> f32 {
        buf_get_do!(self, index, f32, be);
    }

    pub fn get_f32_le(&mut self, index: usize) -> f32 {
        buf_get_do!(self, index, f32, le);
    }

    pub fn get_f64(&mut self, index: usize) -> f64 {
        buf_get_do!(self, index, f64, be);
    }

    pub fn get_f64_le(&mut self, index: usize) -> f64 {
        buf_get_do!(self, index, f64, le);
    }

    pub fn get_bytes(&mut self, index: usize, dest: &mut [u8]) -> usize {
        assert!(self.buf.len() > index);
        let copy_len = if (index + dest.len()) <= self.buf.len() {
            dest.len()
        } else {
            self.buf.len() - index
        };
        dest[..copy_len].copy_from_slice(&self.buf[index..(index + copy_len)]);
        copy_len
    }

    pub fn write_u8(&mut self, val: u8) {
        assert!(self.buf.len() >= (self.writer_index + 1));
        self.buf[self.writer_index] = val;
        self.writer_index += 1;
    }

    pub fn write_u16(&mut self, val: u16) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_u16_le(&mut self, val: u16) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_i16(&mut self, val: i16) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_i16_le(&mut self, val: i16) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_u32_le(&mut self, val: u32) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_i32(&mut self, val: i32) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_i32_le(&mut self, val: i32) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_u64(&mut self, val: u64) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_u64_le(&mut self, val: u64) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_i64(&mut self, val: i64) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_i64_le(&mut self, val: i64) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_u128(&mut self, val: u128) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_u128_le(&mut self, val: u128) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_i128(&mut self, val: i128) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_i128_le(&mut self, val: i128) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_f32(&mut self, val: f32) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_f32_le(&mut self, val: f32) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_f64(&mut self, val: f64) {
        self.write_bytes(&val.to_be_bytes());
    }

    pub fn write_f64_le(&mut self, val: f64) {
        self.write_bytes(&val.to_le_bytes());
    }

    pub fn write_bytes(&mut self, src: &[u8]) {
        let end = self.writer_index + src.len();
        assert!(self.buf.len() >= end);
        self.buf[self.writer_index..end].copy_from_slice(src);
        self.writer_index = end;
    }

    pub fn write_bytes_uncheck(&mut self, src: &[u8]) -> usize {
        let copy_len = if (self.writer_index + src.len()) > self.buf.len() {
            self.buf.len() - self.writer_index
        } else {
            src.len()
        };
        self.buf[self.writer_index..].copy_from_slice(&src[..copy_len]);
        copy_len
    }

    pub fn set_u8(&mut self, index: usize, val: u8) {
        assert!(self.buf.len() > index);
        self.buf[index] = val;
    }

    pub fn set_u16(&mut self, index: usize, val: u16) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_u16_le(&mut self, index: usize, val: u16) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_i16(&mut self, index: usize, val: i16) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_i16_le(&mut self, index: usize, val: i16) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_u32(&mut self, index: usize, val: u32) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_u32_le(&mut self, index: usize, val: u32) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_i32(&mut self, index: usize, val: i32) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_i32_le(&mut self, index: usize, val: i32) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_u64(&mut self, index: usize, val: u64) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_u64_le(&mut self, index: usize, val: u64) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_i64(&mut self, index: usize, val: i64) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_i64_le(&mut self, index: usize, val: i64) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_u128(&mut self, index: usize, val: u128) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_u128_le(&mut self, index: usize, val: u128) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_i128(&mut self, index: usize, val: i128) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_i128_le(&mut self, index: usize, val: i128) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_f32(&mut self, index: usize, val: f32) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_f32_le(&mut self, index: usize, val: f32) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_f64(&mut self, index: usize, val: f64) {
        self.set_bytes(index, &val.to_be_bytes());
    }

    pub fn set_f64_le(&mut self, index: usize, val: f64) {
        self.set_bytes(index, &val.to_le_bytes());
    }

    pub fn set_bytes(&mut self, index: usize, src: &[u8]) {
        let end = index + src.len();
        assert!(self.buf.len() >= end);
        self.buf[index..end].copy_from_slice(src);
    }

    pub fn set_reader_index(&mut self, index: usize) {
        assert!(self.buf.len() >= index && index <= self.writer_index);
        self.reader_index = index;
    }

    pub fn reader_index(&self) -> usize {
        self.reader_index
    }

    pub fn set_writer_index(&mut self, index: usize) {
        assert!(self.buf.len() >= index && index >= self.reader_index);
        self.writer_index = index;
    }

    pub fn writer_index(&self) -> usize {
        self.writer_index
    }

    pub fn set_index(&mut self, reader_index: usize, writer_index: usize) {
        assert!(reader_index <= writer_index && self.buf.len() >= writer_index);
        self.reader_index = reader_index;
        self.writer_index = writer_index;
    }

    pub fn clear(&mut self) {
        self.reader_index = 0;
        self.writer_index = 0;
    }

    pub fn remaining(&self) -> usize {
        self.writer_index - self.reader_index
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn as_slice(&mut self) -> &mut [u8] {
        &mut self.buf[self.reader_index..self.writer_index]
    }

    pub fn as_raw_slice(&mut self) -> &mut [u8] {
        self.buf
    }
}

impl std::fmt::Display for BufViewMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reader_index: {}, writer_index: {}, capacity: {}",
            self.reader_index(),
            self.writer_index(),
            self.capacity()
        )
    }
}

impl<'a> Write for BufViewMut<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.write_bytes_uncheck(buf);
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
