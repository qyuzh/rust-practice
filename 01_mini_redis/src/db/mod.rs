use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::time::Instant;

#[derive(Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

impl Db {
    pub fn new() -> Self {
        todo!()
    }
    
    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        todo!()
    }
}

struct Shared {
    state: Mutex<State>,
}

struct State {
    entries: HashMap<String, Entry>,
}

struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}