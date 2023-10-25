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

fn main() {
    decimal2bytes(12345);
    decimal2bytes2(12345);
}