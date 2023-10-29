use std::io::Cursor;

use bytes::{Buf, Bytes};

pub use error::Error;

mod error;

#[derive(Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(i64),
    Null,
    Bulk(Bytes),
    Array(Vec<Frame>),
}

impl Frame {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(src)? {
            // simple strings `+OK\r\n`
            b'+' => {
                get_line(src)?;
                Ok(())
            }
            // simple errors `-Error message\r\n`
            b'-' => {
                get_line(src)?;
                Ok(())
            }
            // integer `:[<+|->]<value>\r\n`
            b':' => {
                let _ = get_decimal(src)?;
                Ok(())
            }
            // bulk `$<length>\r\n<data>\r\n` | `$-1\r\n`
            b'$' => {
                if peek_u8(src)? == b'-' {
                    // Null Bulk strings
                    skip(src, 4) // skip -1\r\n
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;
                    skip(src, len + 2) // skip n + \r\n
                }
            }
            b'*' => {
                let len: usize = get_decimal(src)?.try_into()?;
                for _ in 0..len {
                    Frame::check(src)?;
                }
                Ok(())
            }
            actual => Err(format!(
                "protocol error, invalid frame type byte `{}`",
                actual as char
            )
            .into()),
        }
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        match get_u8(src)? {
            // simple strings `+OK\r\n`
            b'+' => {
                let line = get_line(src)?.to_vec();
                let str = String::from_utf8(line)?;
                Ok(Frame::Simple(str))
            }
            // simple errors `-Error message\r\n`
            b'-' => {
                let line = get_line(src)?.to_vec();
                let str = String::from_utf8(line)?;
                Ok(Frame::Simple(str))
            }
            // integer `:[<+|->]<value>\r\n`
            b':' => {
                let num = get_decimal(src)?;
                Ok(Frame::Integer(num))
            }
            // bulk `$<length>\r\n<data>\r\n` | `$-1\r\n`
            b'$' => {
                if peek_u8(src)? == b'-' {
                    // Null Bulk strings
                    let line = get_line(src)?;
                    if line != b"-1" {
                        return Err("protocol error: invalid bulk data".into());
                    }
                    Ok(Frame::Null)
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;
                    if src.remaining() < len + 2 {
                        return Err(Error::Incomplete);
                    }
                    let data = Bytes::copy_from_slice(&src.chunk()[..len]);
                    skip(src, len + 2)?; // skip n + \r\n
                    Ok(Frame::Bulk(data))
                }
            }
            b'*' => {
                let len: usize = get_decimal(src)?.try_into()?;
                let mut out = Vec::with_capacity(len);
                for _ in 0..len {
                    out.push(Frame::parse(src)?);
                }
                Ok(Frame::Array(out))
            }
            _ => unimplemented!(),
        }
    }
}

impl Frame {
    pub(crate) fn array() -> Frame {
        Frame::Array(vec![])
    }

    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Bulk(bytes));
            }
            _ => panic!("not an array frame"),
        }
    }

    pub(crate) fn push_int(&mut self, value: i64) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Integer(value));
            }
            _ => panic!("not an array frame"),
        }
    }

    pub(crate) fn to_error(&self) -> crate::Error {
        format!("unexpected frame: {:?}", self).into()
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(src.get_u8())
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    let start = src.position() as usize;
    let end = src.get_ref().len();

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(Error::Incomplete)
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<i64, Error> {
    atoi::atoi::<i64>(get_line(src)?).ok_or_else(|| "protocol error; invalid frame format".into())
}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }
    Ok(src.chunk()[0])
}

fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }
    src.advance(n);
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::frame::Frame;

    #[test]
    fn frame_check_simple_str() {
        let frame: &[u8] = b"+OK\r\n";
        let mut buf = Cursor::new(frame);
        match Frame::check(&mut buf) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn frame_check_simple_str_failure() {
        let frame: &[u8] = b"OK\r\n";
        let mut buf = Cursor::new(frame);
        match Frame::check(&mut buf) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn frame_check_array() {
        let mut buf = gen_cursor_with_buf(b"*2\r\n$3\r\nget\r\n$4\r\nname\r\n");
        match Frame::check(&mut buf) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn frame_parse_simple_string() {
        let mut buf = gen_cursor_with_buf(b"+Ok\r\n");
        if let Ok(Frame::Simple(s)) = Frame::parse(&mut buf) {
            assert!(true);
        } else {
            assert!(false)
        }
    }

    fn gen_cursor_with_buf(str: &[u8]) -> Cursor<&[u8]> {
        Cursor::new(str)
    }
}
