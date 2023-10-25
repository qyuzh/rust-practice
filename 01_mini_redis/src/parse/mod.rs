use std::vec::IntoIter;

use bytes::Bytes;

pub use error::Error;

use crate::frame::Frame;

mod error;

/// Redis frame to Redis command
pub struct Parse {
    parts: IntoIter<Frame>,
}

impl Parse {
    pub fn new(frame: Frame) -> Result<Parse, Error> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(format!("protocol error; expected array, got {:?}", frame).into()),
        };
        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    pub fn next(&mut self) -> Result<Frame, Error> {
        self.parts.next().ok_or(Error::EndOfStream)
    }

    pub fn next_string(&mut self) -> Result<String, Error> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => std::str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| "protocol error; invalid string".into()),
            frame => Err(
                format!(
                    "protocol error; expected simple frame or bulk frame, got {:?}",
                    frame
                ).into()),
        }
    }

    pub fn next_bytes(&mut self) -> Result<Bytes, Error> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(
                format!(
                    "protocol error; expected simple frame or bulk frame, got {:?}",
                    frame
                ).into()),
        }
    }

    pub fn next_int(&mut self) -> Result<i64, Error> {
        use atoi::atoi;
        const MSG: &str = "protocol error; invalid number";
        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(data) => atoi::<i64>(data.as_bytes()).ok_or_else(|| MSG.into()),
            Frame::Bulk(data) => atoi::<i64>(&data).ok_or_else(|| MSG.into()),
            frame => Err(format!("protocol error; expected int frame but got {:?}", frame).into()),
        }
    }

    /// Ensure there are no more entries in the array
    pub fn finish(&mut self) -> Result<(), Error> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of frame, but there was more".into())
        }
    }
}