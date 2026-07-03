use chrono::{DateTime, Duration, Timelike, Utc};

use crate::{dto::TimeBucket, entities::click::Model as ClickModel};

pub fn truncate_to_minute(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_second(0).unwrap().with_nanosecond(0).unwrap()
}

pub fn truncate_to_hour(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}

pub fn truncate_to_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}

pub fn count_by_bucket<F>(
    clicks: &[ClickModel],
    truncate: F,
) -> std::collections::HashMap<DateTime<Utc>, i32>
where
    F: Fn(DateTime<Utc>) -> DateTime<Utc>,
{
    let mut counts = std::collections::HashMap::new();
    for click in clicks {
        let dt: DateTime<Utc> = click.clicked_at.into();
        let bucket = truncate(dt);
        *counts.entry(bucket).or_insert(0) += 1;
    }
    counts
}

pub fn fill_minute_buckets(
    counts: std::collections::HashMap<DateTime<Utc>, i32>,
    len: i64,
) -> Vec<TimeBucket> {
    let end = truncate_to_minute(Utc::now());
    let start = end - Duration::minutes(len - 1);

    (0..len)
        .map(|i| {
            let bucket = start + Duration::minutes(i);
            TimeBucket {
                bucket: bucket.to_rfc3339(),
                count: counts.get(&bucket).copied().unwrap_or(0),
            }
        })
        .collect()
}

pub fn fill_hour_buckets(
    counts: std::collections::HashMap<DateTime<Utc>, i32>,
    len: i64,
) -> Vec<TimeBucket> {
    let end = truncate_to_hour(Utc::now());
    let start = end - Duration::hours(len - 1);

    (0..len)
        .map(|i| {
            let bucket = start + Duration::hours(i);
            TimeBucket {
                bucket: bucket.to_rfc3339(),
                count: counts.get(&bucket).copied().unwrap_or(0),
            }
        })
        .collect()
}

pub fn fill_day_buckets(
    counts: std::collections::HashMap<DateTime<Utc>, i32>,
    len: i64,
) -> Vec<TimeBucket> {
    let end = truncate_to_day(Utc::now());
    let start = end - Duration::days(len - 1);

    (0..len)
        .map(|i| {
            let bucket = start + Duration::days(i);
            TimeBucket {
                bucket: bucket.to_rfc3339(),
                count: counts.get(&bucket).copied().unwrap_or(0),
            }
        })
        .collect()
}
