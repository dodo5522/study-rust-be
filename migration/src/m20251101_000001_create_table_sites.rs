use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{Statement, DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

const SCHEMA_NAME: &str = "spec";

impl Iden for Sites {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_NAME,
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
          .get_connection()
          .execute(Statement::from_string(
              DatabaseBackend::Postgres,
              format!("CREATE SCHEMA IF NOT EXISTS {SCHEMA_NAME};"),
          ))
          .await?;

        manager
            .create_table(
                Table::create()
                    .table((Sites::Schema, Sites::Table))
                    .if_not_exists()
                    .col(pk_auto(Sites::Id))
                    .col(string(Sites::DeviceType))
                    .to_owned(),
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
            .await?;

        manager
          .get_connection()
          .execute(Statement::from_string(
              DatabaseBackend::Postgres,
              format!("DROP SCHEMA IF EXISTS {SCHEMA_NAME};"),
          ))
          .await?;

        Ok(())
    }
}

enum Sites {
    Schema,
    Table,
    Id,
    DeviceType,
}
