//! Echo example.
//! Use `nc 127.0.0.1 30000` to connect.

use std::mem::MaybeUninit;

use futures::StreamExt;
use mini_async_runtime_unix::log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use mini_async_runtime_unix::Executor;
use mini_async_runtime_unix::TcpListener;

fn main() {
    let ex = Executor::new();
    ex.block_on(serve);
}

async fn serve() {
    let mut listener = TcpListener::bind("127.0.0.1:30000").unwrap();
    while let Some(ret) = listener.next().await {
        if let Ok((mut stream, addr)) = ret {
            log!("new connection from {}", addr);
            let f = async move {
                #[rustfmt::skip]
                let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
                #[rustfmt::skip]
                let buf = unsafe { &mut *(&mut buf as *mut [MaybeUninit<u8>; 1024] as *mut [u8; 1024]) };

                loop {
                    match stream.read(buf).await {
                        Ok(n) => {
                            log!("Receive: {:?} from {addr}", unsafe {
                                std::str::from_utf8_unchecked(&buf[..n])
                            });

                            if n == 0 {
                                log!("Connection closed");
                                return;
                            }

                            if stream.write_all(&buf[..n]).await.is_err() {
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
