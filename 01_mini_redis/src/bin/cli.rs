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
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> mini_redis::Result<()> {
    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.hostname, cli.port);
    println!("Connect to {}:{}...", cli.hostname, cli.port);
    let mut client = Client::connect(&addr).await?;

    match &cli.command {
        Command::Get { key, } => {
            if let Some(value) = client.get(key).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
    }

    Ok(())
}