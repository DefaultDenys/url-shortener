use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "urls")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub url_short: String,

    #[sea_orm(unique)]
    pub url_original: String,

    pub created_at: String,

    pub click_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[allow(dead_code)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
