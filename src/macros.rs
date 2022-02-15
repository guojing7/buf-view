macro_rules! buf_read_do {
    ($this:ident, $typ: tt, be) => {
        assert!($this.remaining() >= std::mem::size_of::<$typ>());
        let end = $this.reader_index + std::mem::size_of::<$typ>();
        let val = $typ::from_be_bytes($this.buf[$this.reader_index..end].try_into().unwrap());
        $this.reader_index = end;
        return val;
    };

    ($this:ident, $typ: tt, le) => {
        assert!($this.remaining() >= std::mem::size_of::<$typ>());
        let end = $this.reader_index + std::mem::size_of::<$typ>();
        let val = $typ::from_le_bytes($this.buf[$this.reader_index..end].try_into().unwrap());
        $this.reader_index = end;
        return val;
    };
}

macro_rules! buf_get_do {
    ($this:ident, $index: expr, $typ: tt, be) => {
        let end = $index + std::mem::size_of::<$typ>();
        assert!($this.buf.len() >= end);
        let val = $typ::from_be_bytes($this.buf[$index..end].try_into().unwrap());
        return val;
    };

    ($this:ident, $index: expr, $typ: tt, le) => {
        let end = $index + std::mem::size_of::<$typ>();
        assert!($this.buf.len() >= end);
        let val = $typ::from_le_bytes($this.buf[$index..end].try_into().unwrap());
        return val;
    };
}

pub(crate) use {buf_get_do, buf_read_do};
