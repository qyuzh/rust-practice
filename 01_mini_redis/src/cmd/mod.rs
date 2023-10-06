use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;

pub enum Command {
    Get
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        todo!()
    }

    pub async fn apply(self, db: &Db, dst: &Connection) -> crate::Result<()> {
        todo!()
    }
}