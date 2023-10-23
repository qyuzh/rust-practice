pub use config::*;

pub mod server;
mod db;
pub mod connection;
pub mod cmd;
pub mod frame;
pub mod config;
mod parse;
pub mod client;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
