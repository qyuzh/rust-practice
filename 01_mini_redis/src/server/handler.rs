use crate::cmd::Command;
use crate::connection::Connection;
use crate::db::Db;

pub struct Handler {
    pub db: Db,
    pub connection: Connection,
}

impl Handler {
    pub async fn run(&mut self) -> crate::Result<()> {
        loop {
            let maybe_frame = self.connection.read_frame().await?;
            let frame = match maybe_frame {
                Some(frame) => frame,
                None => return Ok(()),
            };
            let cmd = Command::from_frame(frame)?;
            cmd.apply(&self.db, &mut self.connection).await?;
        }
    }
}
