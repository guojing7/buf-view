use buf_view::BufViewMut;

#[test]
fn test_buf_view_mut() {
    let mut buf = [0u8; 16];
    let mut buf_view = BufViewMut::wrap(&mut buf);

    buf_view.write_u8(8);
    buf_view.write_u16(16);
    buf_view.write_u32(32);

    let src = [1, 2, 3, 4];
    buf_view.write_bytes(&src);

    assert_eq!(buf_view.read_u8(), 8);
    assert_eq!(buf_view.read_u16(), 16);
    assert_eq!(buf_view.read_u32(), 32);

    let mut dest = [0u8; 4];
    buf_view.read_bytes(&mut dest);
    assert_eq!(src, dest);

    assert_eq!(buf_view.get_u8(0), 8);
    assert_eq!(buf_view.get_u16(1), 16);
    assert_eq!(buf_view.get_u32(3), 32);
}
