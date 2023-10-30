use std::future::Future;

use tokio::net::TcpListener;

use listener::Listener;

mod listener;

mod handler;

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let mut server = Listener::new(listener);
    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                eprintln!("failed to run server: {err}");
            }
        }
        _ = shutdown => {
            println!("server shutdown gracefully");
        }
    }
}
