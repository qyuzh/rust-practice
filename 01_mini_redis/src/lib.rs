pub use config::*;

pub mod client;
pub mod config;
pub mod server;

mod db;
mod connection;
mod cmd;
mod frame;
mod parse;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
