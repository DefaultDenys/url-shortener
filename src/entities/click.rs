use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "clicks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub url_short: String,

    pub clicked_at: String,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::url::Entity",
        from = "Column::UrlShort",
        to = "super::url::Column::UrlShort"
    )]
    Url
}

impl Related<super::url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Url.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
