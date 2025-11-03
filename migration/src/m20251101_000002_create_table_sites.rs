use sea_orm_migration::{prelude::*, schema::*};

use super::consts::SCHEMA_SPEC;
use super::iden::site::Sites;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Iden for Sites {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_SPEC,
                Self::Table => "sites",
                Self::Id => "id",
                Self::DeviceType => "device_type",
            }
        )
        .unwrap();
    }
}

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
                    .add_column_if_not_exists(string(Sites::DeviceType))
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
