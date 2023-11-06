//! Echo example.
//! Use `nc 127.0.0.1 30000` to connect.

use chrono::prelude::*;
use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use mini_async_runtime_unix::executor::Executor;
use mini_async_runtime_unix::tcp::TcpListener;

fn main() {
    let ex = Executor::new();
    ex.block_on(serve);
}

async fn serve() {
    let mut listener = TcpListener::bind("127.0.0.1:30000").unwrap();
    while let Some(ret) = listener.next().await {
        if let Ok((mut stream, addr)) = ret {
            println!(
                "{}: [TCP] accept a new connection from {} successfully",
                Local::now(),
                addr
            );
            let f = async move {
                let mut buf = [0; 1024]; // initialize with 0
                loop {
                    match stream.read(&mut buf).await {
                        Ok(n) => {
                            println!("{}: Receive: {:?}", Local::now(), unsafe {
                                String::from_utf8_unchecked(buf[..n].to_vec())
                            });
                            if n == 0 || stream.write_all(&buf[..n]).await.is_err() {
                                return;
                            }
                        }
                        Err(_) => {
                            return;
                        }
                    }
                }
            };
            Executor::spawn(f);
        }
    }
}
