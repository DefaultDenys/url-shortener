use chrono::{DateTime, Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder,
};

use crate::{
    dto::TimeBucket,
    entities::click::{
        ActiveModel as ClickActiveModel, Column, Entity as ClickEntity, Model as ClickModel,
    },
    services::{
        count_by_bucket, fill_day_buckets, fill_hour_buckets, fill_minute_buckets,
        truncate_to_day, truncate_to_hour, truncate_to_minute,
    },
};

#[derive(Clone)]
pub struct ClickRepository {
    pub db: DatabaseConnection,
}

impl ClickRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, url_short: String) -> Result<ClickModel, DbErr> {
        let click_model = ClickActiveModel {
            url_short: Set(url_short),
            clicked_at: Set(Utc::now().into()),
            ..Default::default()
        };

        click_model.insert(&self.db).await
    }

    pub async fn find_since(
        &self,
        url_short: &str,
        since: DateTime<Utc>,
    ) -> Result<Vec<ClickModel>, DbErr> {
        ClickEntity::find()
            .filter(Column::UrlShort.eq(url_short))
            .filter(Column::ClickedAt.gte(since))
            .order_by_asc(Column::ClickedAt)
            .all(&self.db)
            .await
    }

    pub async fn clicks_per_minute(&self, url_short: &str) -> Result<Vec<TimeBucket>, DbErr> {
        let since = truncate_to_minute(Utc::now() - Duration::minutes(59));
        let clicks = self.find_since(url_short, since).await?;
        let counts = count_by_bucket(&clicks, truncate_to_minute);

        Ok(fill_minute_buckets(counts, 60))
    }

    pub async fn clicks_per_hour(&self, url_short: &str) -> Result<Vec<TimeBucket>, DbErr> {
        let since = truncate_to_hour(Utc::now() - Duration::hours(23));
        let clicks = self.find_since(url_short, since).await?;
        let counts = count_by_bucket(&clicks, truncate_to_hour);

        Ok(fill_hour_buckets(counts, 24))
    }

    pub async fn clicks_per_day(&self, url_short: &str) -> Result<Vec<TimeBucket>, DbErr> {
        let since = truncate_to_day(Utc::now() - Duration::days(6));
        let clicks = self.find_since(url_short, since).await?;
        let counts = count_by_bucket(&clicks, truncate_to_day);

        Ok(fill_day_buckets(counts, 7))
    }
}
