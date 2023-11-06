# Mini Rust Async Runtime

To show how a runtime works.

## crates

### chrono

Date and Time for Rust

Chrono aims to provide all functionality needed to do correct operations on dates and time in the proleptic Gregorian
calendar.

### tokio::io

Traits, helpers, and type definitions for asynchronous I/O functionality.

**AsyncRead**

```rust
pub trait AsyncRead {
    // Required method
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>
    ) -> Poll<Result<()>>;
}
```

**AsyncWrite**

```rust
pub trait AsyncWrite {
    // Required methods
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<Result<usize, Error>>;
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), Error>>;
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), Error>>;

    // Provided methods
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>]
    ) -> Poll<Result<usize, Error>> { ... }
    fn is_write_vectored(&self) -> bool { ... }
}
```

**ReadBuf**

A wrapper around a byte buffer that is incrementally filled and initiaized.

### futures::Stream

A stream of values produced asynchronously.

### socket2::Socket

Owned wrapper around system socket.

### std:cell::*

**Cell**

1. Provides interior mutability by moving values in and out of the cell.
2. Ensure that there is never more than one reference pointing to the inner value.

**RefCell**

1. Provides "dynamic borrowing".
2. Using `borrow` to obtain an immutable reference
3. Using `borrow_mut` to obtain a mutable reference

### std::io::Read/Write

The `Read` Trait allows for reading bytes from a source.

