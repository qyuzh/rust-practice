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
    
    /// Read bytes from tcp stream, and then convert it to Redis Frame
    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            // convert if possible
            if let Some(frame) = parse_frame(&mut self.read_buf)? {
                return Ok(Some(frame));
            }
            
            // read bytes from tcp stream
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
                for entry in val.iter() {
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
        self.stream.write_all(val.to_string().as_bytes()).await?;
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

/// Convert bytes to frame
fn parse_frame(read_buf: &mut BytesMut) -> crate::Result<Option<Frame>> {
    let mut buf = Cursor::new(&read_buf[..]);
    match Frame::check(&mut buf) {
        Ok(_) => {
            buf.set_position(0);
            let frame = Frame::parse(&mut buf)?;
            
            read_buf.advance(buf.position() as usize);
            Ok(Some(frame))
        }
        Err(frame::Error::Incomplete) => Ok(None),
        Err(e) => Err(e.into())
    }
}
