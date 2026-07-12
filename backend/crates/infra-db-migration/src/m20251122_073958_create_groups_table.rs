use crate::iden::Group;
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Group::Schema, Group::Table))
                    .if_not_exists()
                    .col(string(Group::Group).primary_key())
                    .col(string(Group::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Group::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!("{}.{}", Group::Schema.to_string(), Group::Table.to_string());
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'グループ (e.g. Array, Battery, ...)';",
                    table,
                    Group::Group.to_string()
                ),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Group::Schema, Group::Table))
                    .to_owned(),
            )
            .await
    }
}
