use std::{
    cell::RefCell,
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener as StdTcpListener, TcpStream as StdTcpStream, ToSocketAddrs},
    os::unix::prelude::AsRawFd,
    rc::{Rc, Weak},
    task::Poll,
};

use chrono::prelude::*;
use futures::Stream;
use socket2::{Domain, Protocol, Socket, Type};

use crate::{reactor::get_reactor, reactor::Reactor};

#[derive(Debug)]
pub struct TcpListener {
    reactor: Weak<RefCell<Reactor>>,
    listener: StdTcpListener,
}

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, io::Error> {
        let addr = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "empty address"))?;

        let domain = if addr.is_ipv6() {
            Domain::IPV6
        } else {
            Domain::IPV4
        };

        let sk = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
        sk.set_reuse_address(true)?;
        sk.bind(&socket2::SockAddr::from(addr))?;
        sk.listen(1024)?;

        let reactor = get_reactor();
        reactor.borrow_mut().add(sk.as_raw_fd());

        println!("{}: [TCP] bind with fd {}", Local::now(), sk.as_raw_fd());

        Ok(Self {
            reactor: Rc::downgrade(&reactor),
            listener: sk.into(),
        })
    }
}

impl Stream for TcpListener {
    type Item = io::Result<(TcpStream, SocketAddr)>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        println!("{}: [TCP] try polling", Local::now());

        match self.listener.accept() {
            Ok((stream, addr)) => Poll::Ready(Some(Ok((stream.into(), addr)))),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // modify reactor to register interest
                self.reactor
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .modify_readable(self.listener.as_raw_fd(), cx);

                Poll::Pending
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}

#[derive(Debug)]
pub struct TcpStream {
    stream: StdTcpStream,
}

impl From<StdTcpStream> for TcpStream {
    fn from(stream: StdTcpStream) -> Self {
        get_reactor().borrow_mut().add(stream.as_raw_fd());
        Self { stream }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        println!("{}: drop", Local::now());
        get_reactor().borrow_mut().delete(self.stream.as_raw_fd());
    }
}

impl tokio::io::AsyncRead for TcpStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let fd = self.stream.as_raw_fd();
        println!("{}: [TCP Stream] read for fd {}", Local::now(), fd);
        let b =
            unsafe { &mut *(buf.unfilled_mut() as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]) };

        match self.stream.read(b) {
            Ok(n) => {
                println!(
                    "{}: [TCP Stream] read for fd {} done, {}",
                    Local::now(),
                    fd,
                    n
                );

                buf.advance(n); // the inner buffer has been initialized

                Poll::Ready(Ok(()))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!(
                    "{}: [TCP Stream] read for fd {} done WouldBlock",
                    Local::now(),
                    fd
                );

                // modify reactor to register interest
                get_reactor()
                    .borrow_mut()
                    .modify_readable(self.stream.as_raw_fd(), cx);

                Poll::Pending
            }
            Err(e) => {
                println!("{}: [TCP Stream] read for fd {} done err", Local::now(), fd);
                Poll::Ready(Err(e))
            }
        }
    }
}

impl tokio::io::AsyncWrite for TcpStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match self.stream.write(buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                get_reactor()
                    .borrow_mut()
                    .modify_writable(self.stream.as_raw_fd(), cx);

                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        self.stream.shutdown(std::net::Shutdown::Write)?;
        Poll::Ready(Ok(()))
    }
}
