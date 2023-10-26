use bytes::{Buf, BufMut, BytesMut};

fn decimal2bytes(val: i64) {
    use std::io::Write;

    let mut buf = [0u8; 20];
    let mut buf = std::io::Cursor::new(&mut buf[..]);
    write!(&mut buf, "{}", val).unwrap();
    let pos = buf.position() as usize;
    println!("{:?}", &buf.get_ref()[..pos]);
}

fn decimal2bytes2(val: i64) {
    println!("{:?}", val.to_string().as_bytes())
}

fn cursor_learning() {
    let mut buffer = BytesMut::with_capacity(32);
    buffer.put(&b"123456789"[..]);
    buffer.advance(3);
    println!("{buffer:?}");
    
    let mut buf = std::io::Cursor::new(&buffer[..]);

    buf.get_mut().advance(3);
    println!("{} {buf:?}", buf.position());
    
    buf.advance(1);
    println!("{} {buf:?}", buf.position());
    
    buf.set_position(2);
    println!("{} {buf:?}", buf.position());

    buf.advance(1);
    println!("{} {buf:?}", buf.position());

    println!("{buffer:?}");
}

fn main() {
    // decimal2bytes(12345);
    // decimal2bytes2(12345);
    cursor_learning();
}