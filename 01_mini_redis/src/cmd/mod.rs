mod get;

use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;

/// Redis Command is Redis Frame::Array variant according to Redis protocol
pub enum Command {
    Get
}

impl Command {
    
    /// Get Command from Frame::Array
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        todo!()
    }

    pub async fn apply(self, db: &Db, dst: &Connection) -> crate::Result<()> {
        todo!()
    }
}