use sea_orm_migration::{prelude::*, schema::*};
use crate::iden::generation::Units;
use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Units::Schema, Units::Table))
                    .if_not_exists()
                    .col(
                        integer(Units::Id)
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        string(Units::Unit)
                            .not_null()
                    )
                    .col(
                        timestamp_with_time_zone(Units::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!("{}.{}", Units::Schema.to_string(), Units::Table.to_string());
        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    DbBackend::Postgres,
                    format!("COMMENT ON COLUMN {}.{} IS '単位 (e.g. kWh, V, A, ...)';", table, Units::Unit.to_string()),
                )
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Units::Schema, Units::Table))
                    .to_owned()
            )
            .await
    }
}
