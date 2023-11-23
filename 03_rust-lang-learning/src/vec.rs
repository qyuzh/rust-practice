//! My Vec implementation
//!
//! # Layout
//! - MyVec is covariant over T
//! - MyVec is Send/Sync if T is Send/Sync
//! - Pointer is never null
//!
//! # Methods
//! - grow, cap = 0 | cap = 1 | cap = old_cap * 2;
//! - push,
//! - pop,
//! - insert,
//! - remove,
//!
//! # Deallocating
//! - drop
//!
//! # Deref&DerefMut
//! - deref
//! - deref_mut
//!
//! # IntoIter
//!
//!

use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{copy, read, write, NonNull};

#[allow(unused)]
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        // handle ZST
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };

        // `NonNull::dangling()` means "unallocated" and "zero-sized allocation"
        Self {
            ptr: NonNull::dangling(), // Not null but is Not valid
            len: 0,
            cap,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, e: T) {
        // We should NOT to evaluate the memory
        // f[idx] = e will try to call `drop` on the old value of f[idx]
        // The correct way to do this is with `str::ptr::write()`, which just
        // overwrites the target address with the bits of the value we provide
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            write(self.ptr.as_ptr().add(self.len), e);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // Rust won't just let us dereference the location of memory to move
        // the value out, because that would leave the memory uninitialized.
        // `ptr::read` just copies out the bits from the target address.
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, e: T) {
        assert!(index < self.len, "index out of bounds");

        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index, // f = [0 1 3], index = 1, count = 3 - 1
            );
            write(self.ptr.as_ptr().add(1), e);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = read(self.ptr.as_ptr().add(index));
            copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    fn grow(&mut self) {
        assert_ne!(mem::size_of::<T>(), 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        let e_size = mem::size_of::<T>();
        if self.cap != 0 && e_size != 0 {
            while let Some(_) = self.pop() {}

            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T, // the first
    end: *const T,   // the last + 1
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const T;
                    Some(read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };

        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(read(self.end))
                }
            }
        }
    }
}

impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let vec = mem::ManuallyDrop::new(self);

        let ptr = vec.ptr;
        let cap = vec.cap;
        let len = vec.len;

        IntoIter {
            buf: ptr,
            cap,
            start: ptr.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                ((ptr.as_ptr() as usize) + len) as *const T
            } else if cap == 0 {
                ptr.as_ptr()
            } else {
                unsafe { ptr.as_ptr().add(len) }
            },
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        let e_size = mem::size_of::<T>();
        if self.cap != 0 && e_size != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_basic() {
        let mut t = super::MyVec::new();
        t.push(1);
        t.push(2);
        t.push(3);
        assert_eq!(t.pop(), Some(3));
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn test_zst() {
        let mut t = super::MyVec::new();
        let cnt = 10;
        for _ in 0..cnt {
            t.push(());
        }
        let mut i = 0;
        for _ in t.into_iter() {
            i += 1;
        }
        assert_eq!(i, cnt);
    }
}
