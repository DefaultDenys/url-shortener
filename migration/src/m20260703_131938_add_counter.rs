use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Urls::Table)
                    .add_column(integer(Urls::ClickCount).default(0).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Urls::Table)
                    .drop_column(Urls::ClickCount)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Urls {
    Table,
    ClickCount,
}
