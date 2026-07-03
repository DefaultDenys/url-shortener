use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Clicks::Table)
                    .if_not_exists()
                    .col(pk_auto(Clicks::Id))
                    .col(string(Clicks::UrlShort).not_null())
                    .col(timestamp_with_time_zone(Clicks::ClickedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_clicks_url_short")
                            .from(Clicks::Table, Clicks::UrlShort)
                            .to(Urls::Table, Urls::UrlShort)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_clicks_url_short_clicked_at")
                    .table(Clicks::Table)
                    .col(Clicks::UrlShort)
                    .col(Clicks::ClickedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_clicks_url_short_clicked_at")
                    .table(Clicks::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Clicks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Clicks {
    Table,
    Id,
    UrlShort,
    ClickedAt,
}

#[derive(DeriveIden)]
enum Urls {
    Table,
    UrlShort,
}