use sea_orm_migration::{prelude::*, schema::*};

use super::iden::spec::Sites;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Sites::Schema, Sites::Table))
                    .if_not_exists()
                    .col(string(Sites::Id))
                    .primary_key(Index::create().col(Sites::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table((Sites::Schema, Sites::Table))
                    .add_column_if_not_exists(string(Sites::Name))
                    .add_column_if_not_exists(string(Sites::Address))
                    .add_column_if_not_exists(double(Sites::Longitude))
                    .add_column_if_not_exists(double(Sites::Latitude))
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Sites::Schema, Sites::Table))
                    .if_exists()
                    .to_owned())
            .await
    }
}
