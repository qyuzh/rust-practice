use std::io;
use std::io::Cursor;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

use crate::frame::Frame;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    read_buf: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(socket),
            read_buf: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            if let Some(frame) = parse_frame(&mut self.read_buf)? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.read_buf).await? {
                return if self.read_buf.is_empty() {
                    Ok(None)
                } else {
                    Err("Connection reset by peer".into())
                };
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        todo!()
    }
}

fn parse_frame(read_buf: &mut BytesMut) -> crate::Result<Option<Frame>> {
    let mut buf = Cursor::new(&read_buf[..]);
    match Frame::check(&mut buf) {
        Ok(_) => {
            let len = buf.position() as usize;
            buf.set_position(0);
            let frame = Frame::parse(&mut buf)?;
            read_buf.advance(len);
            Ok(Some(frame))
        }
        _ => {
            todo!()
        }
    }
}

