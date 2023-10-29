use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::Notify;
use tokio::time;
use tokio::time::Instant;

pub struct DbDropGuard {
    db: Db,
}

impl DbDropGuard {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Drop for DbDropGuard {
    fn drop(&mut self) {
        self.db.shut_down_purging_task();
    }
}

#[derive(Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

impl Db {
    pub fn new() -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                expirations: BTreeSet::new(),
                shutdown: false,
            }),
            purging_task: Notify::new(),
        });

        tokio::spawn(task_for_purging_expired_keys(shared.clone()));

        Db { shared }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|e| e.data.clone())
    }

    /// 1. now = 0,  key = qyuzh, duration = 20, so we have (when=20, qyuzh)
    /// 2. now = 10, key = qyuzh, duration = 10, so we have (when=20, qyuzh)
    pub fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();

        let mut notify = false;
        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;

            notify = state
                .next_expiration()
                .map(|exp| exp > when)
                .unwrap_or(true);

            // insert expiration for current key
            state.expirations.insert((when, key.clone()));

            when
        });

        let prev = state.entries.insert(
            key.clone(),
            Entry {
                data: value,
                expires_at,
            },
        );

        // remove the previous expiration if has for avoiding memory leaking
        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                if expires_at.is_none() || when != expires_at.unwrap() {
                    state.expirations.remove(&(when, key));
                }
            }
        }

        drop(state);

        if notify {
            self.shared.purging_task.notify_one();
        }
    }
}

impl Db {
    fn shut_down_purging_task(&self) {
        let mut state = self.shared.state.lock().unwrap();
        state.shutdown = true;

        // drop in advance for reducing lock contention by ensuring the background task doesn't
        // wake up only to be unable to acquire the mutex
        drop(state);
        self.shared.purging_task.notify_one();
    }
}

struct Shared {
    state: Mutex<State>,
    purging_task: Notify,
}

impl Shared {
    fn purge_expired_keys(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();
        if state.shutdown {
            return None;
        }

        // get the mutable ref to the inner state for avoiding borrow checking failed
        // SAFETY: we have acquired the lock
        let state = &mut *state;

        let now = Instant::now();
        while let Some(&(when, ref key)) = state.expirations.iter().next() {
            if when > now {
                return Some(when);
            }
            state.entries.remove(key);
            state.expirations.remove(&(when, key.clone()));
        }

        None
    }

    fn is_shutdown(&self) -> bool {
        self.state.lock().unwrap().shutdown
    }
}

struct State {
    entries: HashMap<String, Entry>,
    expirations: BTreeSet<(Instant, String)>,
    shutdown: bool,
}

impl State {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations.iter().next().map(|exp| exp.0)
    }
}

struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}

/// Start the task when new Db, and remove the task when all db dropped in DbDropGuard
async fn task_for_purging_expired_keys(shared: Arc<Shared>) {
    while !shared.is_shutdown() {
        if let Some(when) = shared.purge_expired_keys() {
            tokio::select! {
                _ = time::sleep_until(when) => {}
                _ = shared.purging_task.notified() => {}
            }
        } else {
            shared.purging_task.notified().await;
        }
    }
    println!("task_for_purging_expired_keys exit");
}
