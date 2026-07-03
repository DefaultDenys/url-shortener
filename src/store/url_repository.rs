use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::entities::url::{self, Entity as UrlEntity, Model as UrlModel};
#[derive(Clone)]
pub struct UrlRepository {
    db: DatabaseConnection,
}

impl UrlRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_short_url(&self, url_short: &str) -> Option<UrlModel> {
        UrlEntity::find_by_id(url_short.to_string())
            .one(&self.db)
            .await
            .ok()
            .flatten()
    }

    pub async fn find_by_original_url(&self, url_original: &str) -> Option<UrlModel> {
        UrlEntity::find()
            .filter(url::Column::UrlOriginal.eq(url_original))
            .one(&self.db)
            .await
            .ok()
            .flatten()
    }

    pub async fn insert(&self, url_original: String, url_short: String) -> Result<UrlModel, DbErr> {
        let active_model = url::ActiveModel {
            url_short: Set(url_short),
            url_original: Set(url_original),
            click_count: Set(0),
            created_at: Set(chrono::Utc::now().to_rfc3339()),
            ..Default::default()
        };

        active_model.insert(&self.db).await
    }

    pub async fn increment_click_count(&self, url_short: &str) -> Result<(), DbErr> {
        UrlEntity::update_many()
            .col_expr(
                url::Column::ClickCount,
                Expr::col(url::Column::ClickCount).add(1),
            )
            .filter(url::Column::UrlShort.eq(url_short))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
