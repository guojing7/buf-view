use crate::macros::{buf_get_do, buf_read_do};

/// Wrap a &\[u8\] buffer as read only.
///
/// BufView support many methods to read primitive types from a byte buffer easily,
/// it support to read primitive types as big endian or little endian. BufView wrap
/// the original buffer with reader_index and writer_index as fllowing structure.
/// When read data from it, the reader_index will advance the data length, and the
/// read must between the reader_index and writer_index.
///
/// Any more, it support get method, too. It make you random get data from the BufView.
///
/// BufView structure
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
/// use buf_view::BufView;
///
/// let buf = [0, 1, 2, 3, 4, 5, 6, 7];
/// let mut buf_view = BufView::wrap(&buf);
///
/// assert_eq!(buf_view.read_u8(), 0);
/// assert_eq!(buf_view.read_u16(), 0x0102);
/// assert_eq!(buf_view.read_u32_le(), 0x06050403);
/// assert_eq!(buf_view.get_u16_le(1), 0x0201);
///
/// // wrap from vector
/// let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
/// let mut buf_view = BufView::wrap(v.as_slice());
/// assert_eq!(buf_view.read_u8(), 0);
/// assert_eq!(buf_view.read_u32(), 0x01020304);
/// assert_eq!(buf_view.read_u16_le(), 0x0605);
///
/// // wrap from &str
/// let s = "01234567";
/// let mut buf_view = BufView::wrap(s.as_bytes());
/// assert_eq!(buf_view.read_u8(), 0x30);
/// assert_eq!(buf_view.read_u32(), 0x31323334);
/// ```
///
#[derive(Debug)]
pub struct BufView<'a> {
    buf: &'a [u8],
    reader_index: usize,
    writer_index: usize,
}

impl<'a> BufView<'a> {
    /// Wrap the `buf` as BufView, set the reader_index=0 and writer_index=buf.len(),
    /// this make the whole `buf` can read and get by default.
    pub fn wrap(buf: &'a [u8]) -> Self {
        let len = buf.len();
        BufView::wrap_with(buf, 0, len)
    }

    /// Wrap the `buf` as BufView, and specify the reader_index and writer_index.
    /// ```
    /// use buf_view::BufView;
    ///
    /// let buf = [0, 1, 2, 3, 4, 5, 6];
    /// let mut buf = BufView::wrap_with(&buf, 1, 5);
    /// assert_eq!(buf.read_u32(), 0x01020304);
    /// ```
    pub fn wrap_with(buf: &'a [u8], reader_index: usize, writer_index: usize) -> Self {
        assert!(reader_index <= writer_index && buf.len() >= writer_index);
        BufView {
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

    pub fn as_slice(&mut self) -> &[u8] {
        &self.buf[self.reader_index..self.writer_index]
    }

    pub fn as_raw_slice(&mut self) -> &[u8] {
        self.buf
    }
}

impl std::fmt::Display for BufView<'_> {
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
