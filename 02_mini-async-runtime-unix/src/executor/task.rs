use std::{
    cell::RefCell,
    mem,
    rc::Rc,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use futures::future::LocalBoxFuture;

use super::EX;
use crate::log;

pub struct Task {
    pub future: RefCell<LocalBoxFuture<'static, ()>>,
    pub id: usize,
}

impl Task {
    fn wake_(self: Rc<Self>) {
        Self::wake_by_ref_(&self)
    }

    fn wake_by_ref_(self: &Rc<Self>) {
        EX.with(|ex| ex.local_queue.push(self.clone()));
    }

    /// Return a `Waker` that can be used to wake up the task.
    pub fn waker(wake: Rc<Task>) -> Waker {
        let ptr = Rc::into_raw(wake) as *const ();
        let vtable = &Helper::VTABLE;
        unsafe { Waker::from_raw(RawWaker::new(ptr, vtable)) }
    }

    pub fn new(future: LocalBoxFuture<'static, ()>) -> Self {
        static ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Task {
            id,
            future: RefCell::new(future),
        }
    }

    pub fn poll(self: &Rc<Self>) -> std::task::Poll<()> {
        log!("polling task id={}", self.id);
        let mut future = self.future.borrow_mut();
        let waker = Self::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        match future.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(()) => {
                log!("task id={} completed", self.id);
                std::task::Poll::Ready(())
            }
            std::task::Poll::Pending => {
                log!("task id={} is pending", self.id);
                std::task::Poll::Pending
            }
        }
    }
}

struct Helper;

impl Helper {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_waker,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone_waker(data: *const ()) -> RawWaker {
        increase_refcount(data);
        let vtable = &Self::VTABLE;
        RawWaker::new(data, vtable)
    }

    unsafe fn wake(ptr: *const ()) {
        let rc = Rc::from_raw(ptr as *const Task);
        rc.wake_();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let rc = mem::ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
        rc.wake_by_ref_();
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(Rc::from_raw(ptr as *const Task));
    }
}

unsafe fn increase_refcount(data: *const ()) {
    let rc = mem::ManuallyDrop::new(Rc::<Task>::from_raw(data as *const Task));
    let _rc_clone = rc.clone();
}
