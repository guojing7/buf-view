# buf-view

A utility library to read and write primitive types on a wrapped buffer.

## usage

To use `buf-view`, first add this to your `Cargo.toml`:

```toml
[dependencies]
buf-view = "0.1.0"
```

## BufView

Wrap a buffer to read only.

```rust
use buf_view::BufView;

let buf = [0, 1, 2, 3, 4, 5, 6, 7];
let mut buf_view = BufView::wrap(&buf);

assert_eq!(buf_view.read_u8(), 0);
assert_eq!(buf_view.read_u16(), 0x0102);
assert_eq!(buf_view.read_u32_le(), 0x06050403);
assert_eq!(buf_view.get_u16_le(1), 0x0201);

// wrap from vector
let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
let mut buf_view = BufView::wrap(v.as_slice());
assert_eq!(buf_view.read_u8(), 0);
assert_eq!(buf_view.read_u32(), 0x01020304);

// wrap from &str
let s = "01234567";
let mut buf_view = BufView::wrap(s.as_bytes());
assert_eq!(buf_view.read_u8(), 0x30);
assert_eq!(buf_view.read_u32(), 0x31323334);
```

## BufViewMut

Wrap a buffer to read and write.

```rust
use buf_view::BufViewMut;

let mut buf = [0u8;7];
let mut buf_view = BufViewMut::wrap(&mut buf);

buf_view.write_u8(0);
buf_view.write_u16(0x0102);
buf_view.write_u32(0x03040506);

assert_eq!(buf_view.read_u8(), 0);
assert_eq!(buf_view.read_u16(), 0x0102);
assert_eq!(buf_view.read_u32(), 0x03040506);
assert_eq!(buf_view.get_u16(1), 0x0102);
```

## License

This project is licensed under the [MIT license](https://opensource.org/licenses/MIT).

### Contribution

All contributions are welcomed!
