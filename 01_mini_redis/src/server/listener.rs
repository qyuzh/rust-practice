use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, info_span, instrument, Instrument};

use crate::connection::Connection;
use crate::db::DbDropGuard;

use super::handler::Handler;

pub struct Listener {
    db_holder: DbDropGuard,
    listener: TcpListener,
}

impl Listener {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            db_holder: DbDropGuard::new(),
            listener,
        }
    }

    #[instrument(name = "listener", skip(self))]
    pub async fn run(&mut self) -> crate::Result<()> {
        // accept a socket, then handle it
        loop {
            let (socket, addr) = self.accept().await?;

            info!("accepted a new connection from {}", addr);

            let mut handler = Handler::new(self.db_holder.db(), Connection::new(socket));

            let f = async move {
                info!("handle start");
                if let Err(err) = handler.run().await {
                    error!(%err);
                }
                info!("handle end  ");
            };

            tokio::spawn(f.instrument(info_span!("handler", peer_addr = %addr)));
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
