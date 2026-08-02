use crate::iden::Group;
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

pub(crate) async fn create_groups_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let table = format!("{}.{}", Group::Schema.to_string(), Group::Table.to_string());
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

pub(crate) async fn drop_groups_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(
            Table::drop()
                .table((Group::Schema, Group::Table))
                .to_owned(),
        )
        .await
}
