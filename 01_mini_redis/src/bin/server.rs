use time::macros::format_description;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::fmt;

use mini_redis::server;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    config_logger()?;

    let port = mini_redis::DEFAULT_PORT;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}

fn config_logger() -> mini_redis::Result<()> {
    let timer = fmt::time::LocalTime::new(
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]")
    );
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_timer(timer)
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    Ok(tracing::subscriber::set_global_default(subscriber)?)
}
