use std::vec::IntoIter;
use bytes::Bytes;

use error::Error;

use crate::frame::Frame;

mod error;

pub struct Parse {
    parts: IntoIter<Frame>,
}

impl Parse {
    pub fn new(frame: Frame) -> Result<Parse, Error> {
        todo!()
    }
    
    pub fn next(&mut self) -> Result<Frame, Error> {
        todo!()
    }
    
    pub fn next_string(&mut self) -> Result<String, Error> {
        todo!()
    }
    
    pub fn next_bytes(&mut self) -> Result<Bytes, Error> {
        todo!()
    }
    
    pub fn next_int(&mut self) -> Result<i64, Error> {
        todo!()
    }
    
    /// Ensure there are no more entries in the array
    pub fn finish(&mut self) -> Result<(), Error> {
        todo!()
    }
}