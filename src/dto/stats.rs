use serde::Serialize;

#[derive(Serialize)]
pub struct StatsResponse {
    pub url_short: String,
    pub url_original: String,
    pub click_count: i32,
    pub created_at: String,
    pub clicks_per_minute: Vec<TimeBucket>,
    pub clicks_per_hour: Vec<TimeBucket>,
    pub clicks_per_day: Vec<TimeBucket>,
}

#[derive(Serialize)]
pub struct TimeBucket {
    pub bucket: String,
    pub count: i32,
}
