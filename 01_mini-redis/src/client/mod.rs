use std::io::ErrorKind;
use std::time::Duration;

use bytes::Bytes;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::cmd::get::Get;
use crate::cmd::set::Set;
use crate::connection::Connection;
use crate::frame::Frame;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }

    pub async fn get(&mut self, key: &str) -> crate::Result<Option<Bytes>> {
        let frame = Get::new(key).into_frame();
        self.connection.write_frame(&frame).await?;
        match self.read_response().await? {
            Frame::Simple(value) => Ok(Some(value.into())),
            Frame::Bulk(value) => Ok(Some(value)),
            Frame::Null => Ok(None),
            frame => Err(frame.to_error()),
        }
    }

    pub async fn set(
        &mut self,
        key: &str,
        value: Bytes,
        expire: Option<Duration>,
    ) -> crate::Result<()> {
        let frame = Set::new(key, value, expire).into_frame();
        self.connection.write_frame(&frame).await?;
        match self.read_response().await? {
            Frame::Simple(resp) if resp == "OK" => Ok(()),
            frame => Err(frame.to_error()),
        }
    }
}

impl Client {
    async fn read_response(&mut self) -> crate::Result<Frame> {
        let resp = self.connection.read_frame().await?;
        match resp {
            Some(Frame::Error(msg)) => Err(msg.into()),
            Some(frame) => Ok(frame),
            None => {
                let err =
                    std::io::Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                Err(err.into())
            }
        }
    }
}
