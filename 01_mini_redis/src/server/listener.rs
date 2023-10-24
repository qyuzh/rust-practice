use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};

use crate::connection::Connection;
use crate::db::Db;

use super::handler::Handler;

pub struct Listener {
    pub db: Db,
    pub listener: TcpListener,
}

impl Listener {
    pub async fn run(&mut self) -> crate::Result<()> {
        // accept a socket, then handle it
        loop {
            let (socket, _) = self.accept().await?;

            let mut handler = Handler {
                db: self.db.clone(),
                connection: Connection::new(socket),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    println!("{err}");
                }
            });
        }
    }
}

impl Listener {
    async fn accept(&mut self) -> crate::Result<(TcpStream, SocketAddr)> {
        let mut backoff = 1;
        loop {
            match self.listener.accept().await {
                Ok((socket, addr)) => return Ok((socket, addr)),
                Err(err) => {
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(backoff)).await;
            backoff *= 2;
        }
    }
}