use bytes::Bytes;

use crate::connection::Connection;
use crate::db::Db;
use crate::frame::Frame;
use crate::parse::Parse;

pub struct Get {
    pub key: String,
}

impl Get {
    pub(crate) fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }
    
    fn key(&self) -> &str {
        &self.key
    }
}

impl Get {
    pub(crate) fn parse_frame(parse: &mut Parse) -> crate::Result<Get> {
        let key = parse.next_string()?;
        Ok(Get { key })
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
}

impl Get {
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("get".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame
    }
}
