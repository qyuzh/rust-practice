use std::io;
use std::io::Cursor;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use crate::frame;
use crate::frame::Frame;

/// Tcp bytes to Redis frame, vice versa.
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
        match frame {
            Frame::Array(val) => {
                self.stream.write_u8(b'*').await?;
                self.write_decimal(val.len() as i64).await?;
                for entry in &**val {
                    self.write_value(entry).await?;
                }
            }
            _ => self.write_value(frame).await?,
        }
        self.stream.flush().await
    }
}

impl Connection {
    async fn write_decimal(&mut self, val: i64) -> io::Result<()> {
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }

    async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();
                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as i64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unreachable!(),
        }

        Ok(())
    }
}

fn parse_frame(read_buf: &mut BytesMut) -> crate::Result<Option<Frame>> {
    let mut buf = Cursor::new(&read_buf[..]);
    match Frame::check(&mut buf) {
        Ok(_) => {
            let len = buf.position() as usize;
            buf.set_position(0);
            // buf.get_ref().advance(1);
            let frame = Frame::parse(&mut buf)?;
            read_buf.advance(len);
            Ok(Some(frame))
        }
        Err(frame::Error::Incomplete) => Ok(None),
        Err(e) => Err(e.into())
    }
}
