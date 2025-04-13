#![allow(unused)]

pub mod executor;
pub mod tcp;

pub use executor::Executor;
pub use tcp::TcpListener;
pub use tcp::TcpStream;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        println!("{}:{}: {}",
            // chrono::Local::now(),
            file!(),
            line!(),
            format_args!($($arg)*)
        );
    }};
}
