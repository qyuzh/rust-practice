use std::convert::Infallible;
use std::num::ParseIntError;
use std::time::Duration;

use bytes::Bytes;
use clap::{Parser, Subcommand};

use mini_redis::client::Client;
use mini_redis::config::DEFAULT_PORT;

#[derive(Parser)]
#[command(name = "redis-cli", version, author, about = "Redis Cli")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    hostname: String,

    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Subcommand)]
enum Command {
    /// Get the value of key.
    Get {
        /// Name of key to get
        key: String,
    },
    /// Set key to hold the string value.
    Set {
        /// Name of key to set
        key: String,

        /// Value to set.
        #[clap(value_parser = bytes_from_str)]
        value: Bytes,

        /// Expire the value after specified amount of time
        #[clap(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> mini_redis::Result<()> {
    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.hostname, cli.port);
    let mut client = Client::connect(&addr).await?;

    match cli.command {
        Command::Get { key, } => {
            print!("Get \"{key}\":...");
            if let Some(value) = client.get(&key).await? {
                if let Ok(string) = std::str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("Nil: (nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            print!("Set \"{key}:{}\"...", std::str::from_utf8(&value)?);
            client.set(&key, value).await?;
            println!("OK");
        }
        _ => {}
    }

    Ok(())
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<i64>()?;
    Ok(Duration::from_millis(ms as u64))
}

fn bytes_from_str(src: &str) -> Result<Bytes, Infallible> {
    Ok(Bytes::from(src.to_string()))
}