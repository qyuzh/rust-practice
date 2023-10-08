use get::Get;

use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;
use crate::parse::Parse;

mod get;

/// Redis Command is Redis Frame::Array variant according to Redis protocol
pub enum Command {
    Get(Get),
    Other,
}

impl Command {
    /// Get Command from Frame::Array
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        // 1. get a parser that parse the frame
        let mut parse = Parse::new(frame)?;

        // 2. get the command name
        let command_name = parse.next_string()?.to_lowercase();

        // 3. get the command according to command name
        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frame(&mut parse)?),
            _ => todo!()
        };

        // 4. check if finished
        parse.finish()?;

        // 5. return
        Ok(command)
    }

    /// Apply the command to the specified Db instance. The response is written to dst.
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        use Command::*;
        match self {
            Get(cmd) => cmd.apply(db, dst).await,
            _ => todo!()
        }
    }
}