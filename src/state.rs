use std::sync::{Arc, Mutex};

use crate::InMemoryStore;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<InMemoryStore>>,
}

impl AppState {
    pub fn new(store: InMemoryStore) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
        }
    }
}
