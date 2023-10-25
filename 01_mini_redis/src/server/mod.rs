use std::future::Future;

use tokio::net::TcpListener;

use listener::Listener;

use crate::db::DbDropGuard;

mod listener;

mod handler;

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let mut server = Listener {
        db_holder: DbDropGuard::new(),
        listener,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                eprintln!("Failed to run server: {err}");
            }
        }
        _ = shutdown => {
            println!("shutdown");
        }
    }
}
