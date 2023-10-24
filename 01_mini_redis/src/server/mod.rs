use std::future::Future;

use tokio::net::TcpListener;

use listener::Listener;

use crate::db::Db;

mod listener;

mod handler;

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let mut server = Listener {
        db: Db::new(),
        listener,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                todo!();
            }
        }
        _ = shutdown => {
            todo!();
        }
    }
}
