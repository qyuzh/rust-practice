use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;
use crate::parse::Parse;

struct Get {
    key: String,
}

impl Get {
    fn new(key: impl ToString) -> Get {
        todo!()
    }

    fn key(&self) -> &str {
        &self.key
    }

    async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        todo!()
    }

    fn into_frame(self) -> Frame {
        todo!()
    }
}

impl Get {
    fn parse_frame(parse: &mut Parse) -> crate::Result<Get> {
        todo!()
    }
}