use std::{cell::RefCell, pin::Pin, rc::Rc, task::Context};

use chrono::prelude::*;
use futures::{Future, FutureExt};

use crate::log;
use reactor::Reactor;
use task::*;
use task_queue::*;

mod reactor;
mod task;
mod task_queue;

scoped_tls::scoped_thread_local!(pub static EX: Executor);

pub struct Executor {
    local_queue: TaskQueue,
    pub reactor: Rc<RefCell<Reactor>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            local_queue: TaskQueue::default(),
            reactor: Rc::new(RefCell::new(Reactor::default())),
        }
    }

    pub fn get_reactor() -> Rc<RefCell<Reactor>> {
        EX.with(|ex| ex.reactor.clone())
    }

    pub fn spawn(fut: impl Future<Output = ()> + 'static) {
        let t = Rc::new(Task::new(fut.boxed_local()));

        EX.with(|ex| ex.local_queue.push(t));
    }

    pub fn block_on<F, T>(&self, f: F)
    where
        F: Fn() -> T,
        T: Future<Output = ()> + 'static,
    {
        let fut = f();
        let fut = Rc::new(Task::new(fut.boxed_local()));

        EX.set(self, || {
            loop {
                // return if the outer future is ready
                if let std::task::Poll::Ready(t) = fut.poll() {
                    break t;
                }

                // consume all tasks
                let mut cnt = 0;
                while let Some(t) = self.local_queue.pop() {
                    t.poll();
                    cnt += 1;
                }

                if cnt > 0 {
                    log!("executed {} tasks", cnt);
                } else {
                    log!("no task to execute");
                }

                // no task to execute now, it may ready
                if let std::task::Poll::Ready(t) = fut.poll() {
                    break t;
                }

                // block for io
                self.reactor.borrow_mut().wait();
            }
        })
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
