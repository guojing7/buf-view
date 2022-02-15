use buf_view::BufView;

#[test]
fn test_buf_view() {
    let buf = [8, 0, 16, 0, 0, 0, 32, 1, 2, 3, 4];
    let mut buf_view = BufView::wrap(&buf);

    assert_eq!(buf_view.read_u8(), 8);
    assert_eq!(buf_view.read_u16(), 16);
    assert_eq!(buf_view.read_u32(), 32);

    let src = [1, 2, 3, 4];
    let mut dest = [0; 4];
    buf_view.read_bytes(&mut dest);
    assert_eq!(src, dest);

    assert_eq!(buf_view.get_u8(0), 8);
    assert_eq!(buf_view.get_u16(1), 16);
    assert_eq!(buf_view.get_u32(3), 32);

    let buf = [0, 1, 2, 3, 4, 5, 6];
    let mut buf_view = BufView::wrap_with(&buf, 1, 5);
    assert_eq!(buf_view.read_u32(), 0x01020304);

    let buf = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut buf_view = BufView::wrap(buf.as_slice());
    assert_eq!(buf_view.read_u8(), 0);
    assert_eq!(buf_view.read_u32(), 0x01020304);

    let s = "01234567";
    let mut buf_view = BufView::wrap(s.as_bytes());
    assert_eq!(buf_view.read_u8(), 0x30);
    assert_eq!(buf_view.read_u32(), 0x31323334);
}
