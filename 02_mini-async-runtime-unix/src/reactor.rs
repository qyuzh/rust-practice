use std::{
    cell::RefCell,
    os::unix::prelude::RawFd,
    rc::Rc,
    task::{Context, Waker},
};

use chrono::prelude::*;
use polling::{Event, Poller};
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Reactor {
    poller: Poller,
    waker_mapping: FxHashMap<u64, Waker>,
    buffer: Vec<Event>,
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            poller: Poller::new().unwrap(),
            waker_mapping: Default::default(),
            buffer: Vec::with_capacity(2048),
        }
    }

    // Epoll related
    pub fn add(&mut self, fd: RawFd) {
        println!("{}: [reactor] add fd {}", Local::now(), fd);

        let flags = nix::fcntl::fcntl(fd, nix::fcntl::F_GETFL).unwrap();
        let flags = nix::fcntl::OFlag::from_bits(flags).unwrap();

        let flags_nonblocking = flags | nix::fcntl::OFlag::O_NONBLOCK;
        nix::fcntl::fcntl(fd, nix::fcntl::F_SETFL(flags_nonblocking)).unwrap();

        let event = Event::none(fd as usize);
        self.poller.add(fd, event).unwrap();
    }

    pub fn modify_readable(&mut self, fd: RawFd, cx: &mut Context) {
        println!(
            "{}: [reactor] modify_readable fd {}, token {}",
            Local::now(),
            fd,
            fd * 2
        );

        self.push_completion(fd as u64 * 2, cx);

        let event = Event::readable(fd as usize);
        self.poller.modify(fd, event);
    }

    pub fn modify_writable(&mut self, fd: RawFd, cx: &mut Context) {
        println!(
            "{}: [reactor] modify_writable fd {}, token {}",
            Local::now(),
            fd,
            fd * 2 + 1
        );

        self.push_completion(fd as u64 * 2 + 1, cx);

        let event = Event::writable(fd as usize);
        self.poller.modify(fd, event);
    }

    pub fn wait(&mut self) {
        println!("{}: [reactor] waiting", Local::now());

        self.poller.wait(&mut self.buffer, None); // block if no event emits

        for i in 0..self.buffer.len() {
            let event = self.buffer.swap_remove(0);
            if event.readable {
                if let Some(waker) = self.waker_mapping.remove(&(event.key as u64 * 2)) {
                    println!(
                        "{}: [reactor] fd {}, read waker token {} removed and woken",
                        Local::now(),
                        event.key,
                        event.key * 2
                    );
                    waker.wake();
                }
            }
            if event.writable {
                if let Some(waker) = self.waker_mapping.remove(&(event.key as u64 * 2 + 1)) {
                    println!(
                        "{}: [reactor] fd {}, write waker token {} removed and woken",
                        Local::now(),
                        event.key,
                        event.key * 2 + 1
                    );
                    waker.wake();
                }
            }
        }
    }

    pub fn delete(&mut self, fd: RawFd) {
        println!(
            "{}: [reactor] fd {}, waker token {}, {} removed",
            Local::now(),
            fd,
            fd * 2,
            fd * 2 + 1
        );

        self.waker_mapping.remove(&(fd as u64 * 2));
        self.waker_mapping.remove(&(fd as u64 * 2 + 1));
    }

    fn push_completion(&mut self, token: u64, cx: &mut Context) {
        self.waker_mapping.insert(token, cx.waker().clone());
    }
}

impl Default for Reactor {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
pub fn get_reactor() -> Rc<RefCell<Reactor>> {
    crate::executor::EX.with(|ex| ex.reactor.clone())
}
