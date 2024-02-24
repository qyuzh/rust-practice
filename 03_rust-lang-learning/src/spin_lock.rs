use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }

    /// SAFETY: The &mut T from lock() must be gone!
    /// (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release)
    }

    pub fn lock2(&self) -> Guard<'_, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop()
        }
        Guard { lock: self }
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release)
    }
}
