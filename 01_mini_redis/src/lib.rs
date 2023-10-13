pub use config::*;

pub mod server;
mod db;
mod connection;
mod cmd;
mod frame;
mod config;
mod parse;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
