use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};

use crate::connection::Connection;
use crate::db::{Db, DbDropGuard};

use super::handler::Handler;

pub struct Listener {
    pub db_holder: DbDropGuard,
    pub listener: TcpListener,
}

impl Listener {
    pub async fn run(&mut self) -> crate::Result<()> {
        // accept a socket, then handle it
        loop {
            let (socket, _) = self.accept().await?;

            let mut handler = Handler {
                db: self.db_holder.db(),
                connection: Connection::new(socket),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    eprintln!("{err}");
                }
                println!("{}#{}: peer exit", file!(), line!());
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
