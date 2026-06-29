use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn generate_url_short(url_original: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url_original.hash(&mut hasher);
    format!("{:06x}", hasher.finish() & 0xFFFFFF)
}
