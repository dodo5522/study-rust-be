use sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};
use crate::iden::generation::Labels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Labels::Schema, Labels::Table))
                    .if_not_exists()
                    .col(
                        integer(Labels::Id)
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        string(Labels::Label)
                            .not_null()
                    )
                    .col(
                        timestamp_with_time_zone(Labels::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!("{}.{}", Labels::Schema.to_string(), Labels::Table.to_string());
        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    DbBackend::Postgres,
                    format!("COMMENT ON COLUMN {}.{} IS 'ラベル (e.g. Array Voltage, ...)';", table, Labels::Label.to_string()),
                )
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Labels::Schema, Labels::Table))
                    .to_owned()
            )
            .await
    }
}
