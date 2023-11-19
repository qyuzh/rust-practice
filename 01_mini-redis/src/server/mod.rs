use std::future::Future;

use tokio::net::TcpListener;
use tracing::{error, info};

mod handler;
mod listener;

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let mut server = listener::Listener::new(listener);
    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!(%err);
            }
        }
        _ = shutdown => {
            info!("received shutdown command");
        }
    }
}
