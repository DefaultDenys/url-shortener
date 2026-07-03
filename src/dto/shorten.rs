use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url_original: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub url_short: String,
    pub url_original: String,
    pub click_count: i32,
}
