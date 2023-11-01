use opentelemetry::global;
use time::macros::format_description;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use mini_redis::server;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    config_logger()?;

    let port = mini_redis::DEFAULT_PORT;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    info!("server started");
    server::run(listener, signal::ctrl_c()).await;
    info!("server exited ");

    Ok(())
}

fn config_logger() -> mini_redis::Result<()> {
    let timer = fmt::time::LocalTime::new(
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]")
    );

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("mini-redis")
        .install_simple()?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(
            fmt::Layer::default()
                .compact()
                .with_timer(timer)
                .with_file(true)
                .with_line_number(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .try_init()?;

    Ok(())
}
