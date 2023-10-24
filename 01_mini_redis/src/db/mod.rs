use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|e| e.data.clone())
    }
    
    pub fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();
        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;
            when
        });

        let _prev = state.entries.insert(
            key.clone(),
            Entry {
                data: value,
                expires_at,
            },
        );
        
        // drop(state);
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
