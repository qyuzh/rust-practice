use get::Get;
use set::Set;

use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;
use crate::parse::Parse;

pub mod get;
pub mod set;

/// Redis Command is Redis Frame::Array variant according to Redis protocol
pub enum Command {
    Get(Get),
    Set(Set),
    Other,
}

impl Command {
    /// Get Command from Frame::Array
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;
        let command_name = parse.next_string()?.to_lowercase();
        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frame(&mut parse)?),
            "set" => Command::Set(Set::parse_frame(&mut parse)?),
            _ => unimplemented!(),
        };
        parse.finish()?;
        Ok(command)
    }

    /// Apply the command to the specified Db instance. The response is written to dst.
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        use Command::*;
        match self {
            Get(cmd) => cmd.apply(db, dst).await,
            Set(cmd) => cmd.apply(db, dst).await,
            _ => unimplemented!(),
        }
    }
}
