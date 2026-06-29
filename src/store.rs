use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ShortLink {
    pub url_short: String,
    pub url_original: String,
}

pub trait UrlStore {
    fn insert(&mut self, url_original: String, url_short: String);
    fn lookup(&self, url_short: &str) -> Option<String>;
}

pub struct InMemoryStore {
    map: HashMap<String, String>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl UrlStore for InMemoryStore {
    fn insert(&mut self, url_original: String, url_short: String) {
        self.map.insert(url_short, url_original);
    }

    fn lookup(&self, url_short: &str) -> Option<String> {
        self.map.get(url_short).cloned()
    }
}
