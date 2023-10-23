use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;
use crate::parse::Parse;

pub struct Get {
    key: String,
}

impl Get {
    pub(crate) fn new(key: impl ToString) -> Get {
        todo!()
    }

    fn key(&self) -> &str {
        &self.key
    }

    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let resp = if let Some(v) = db.get(&self.key) {
            Frame::Bulk(v)
        } else {
            Frame::Null
        };
        
        dst.write_frame(&resp).await?;
        
        Ok(())
    }

    pub(crate) fn into_frame(self) -> Frame {
        todo!()
    }
}

impl Get {
    pub(crate) fn parse_frame(parse: &mut Parse) -> crate::Result<Get> {
        todo!()
    }
}