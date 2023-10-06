use tokio::net::TcpListener;
use tokio::signal;

use mini_redis::server;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    let port = mini_redis::DEFAULT_PORT;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    server::run(listener, signal::ctrl_c()).await;
    Ok(())
}