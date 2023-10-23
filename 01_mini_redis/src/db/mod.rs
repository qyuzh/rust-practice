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
        let shared = Arc::new(
            Shared {
                state: Mutex::new(
                    State {
                        entries: HashMap::new()
                    }
                )
            }
        );

        Db { shared }
    }

    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|e| e.data.clone())
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