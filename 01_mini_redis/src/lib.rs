pub use constants::*;

mod cmd;
mod connection;
mod constants;
mod db;
mod frame;
pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
