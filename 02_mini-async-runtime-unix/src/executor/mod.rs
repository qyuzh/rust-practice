use std::{cell::RefCell, pin::Pin, rc::Rc, task::Context};

use chrono::prelude::*;
use futures::{Future, FutureExt};

use task::*;
use task_queue::*;

use crate::reactor::Reactor;

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

    pub fn spawn(fut: impl Future<Output = ()> + 'static) {
        let t = Rc::new(Task {
            future: RefCell::new(fut.boxed_local()),
        });

        EX.with(|ex| ex.local_queue.push(t));
    }

    pub fn block_on<F, T, O>(&self, f: F) -> O
    where
        F: Fn() -> T,
        T: Future<Output = O> + 'static,
    {
        let _waker = waker_fn::waker_fn(|| {});
        let cx = &mut Context::from_waker(&_waker);

        let fut = f();
        pin_utils::pin_mut!(fut);

        // for a duration of a closure
        EX.set(self, || {
            loop {
                // return if the outer future is ready
                if let std::task::Poll::Ready(t) = fut.as_mut().poll(cx) {
                    break t;
                }

                // consume all tasks
                while let Some(t) = self.local_queue.pop() {
                    let w = Task::waker(t.clone());
                    let _wc = w.clone();
                    let mut context = Context::from_waker(&w);

                    let future = t.future.borrow_mut();
                    let _ = Pin::new(future).as_mut().poll(&mut context);
                }

                // no task to execute now, it may ready
                if let std::task::Poll::Ready(t) = fut.as_mut().poll(cx) {
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
