use std::ops::Deref;
use std::ptr;
use std::sync::atomic;
use std::sync::atomic::Ordering;

pub struct MyArc<T> {
    ptr: ptr::NonNull<ArcInner<T>>,
    // phantom: PhantomData<ArcInner<T>>, // The Rustonomicon says this is necessary
}

pub struct ArcInner<T> {
    rc: atomic::AtomicUsize,
    data: T,
}

impl<T> MyArc<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(ArcInner {
            rc: atomic::AtomicUsize::new(1),
            data,
        });

        Self {
            ptr: ptr::NonNull::new(Box::into_raw(boxed)).unwrap(),
            // phantom: PhantomData,
        }
    }
}

unsafe impl<T: Sync + Send> Send for MyArc<T> {}
unsafe impl<T: Sync + Send> Sync for MyArc<T> {}

impl<T> Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };

        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);

        if old_rc > isize::MAX as usize {
            std::process::abort();
        }

        Self {
            ptr: self.ptr,
            // phantom: PhantomData,
        }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        // Can NOT understand this fully.
        // The standard lib says, "This fence is needed to prevent reordering of
        // use of the data and deletion of the data."
        atomic::fence(Ordering::Acquire);
        unsafe {
            let _ = Box::from_raw(self.ptr.as_ptr());
        }
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use crate::arc::MyArc;

    #[test]
    fn test() {
        let t = MyArc::new(2);
        let t1 = t.clone();
        thread::spawn(move || {
            let data = *t1;
            println!("From child thread: {}", data);
        })
        .join()
        .unwrap();

        println!("{}", *t);
    }
}
