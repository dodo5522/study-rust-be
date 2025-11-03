use sea_orm_migration::{prelude::*, schema::*};
use crate::iden::generation::Groups;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Groups::Schema, Groups::Table))
                    .if_not_exists()
                    .col(
                        integer(Groups::Id)
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        string(Groups::Group)
                            .not_null()
                    )
                    .col(
                        timestamp_with_time_zone(Groups::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Groups::Schema, Groups::Table))
                    .to_owned()
            )
            .await
    }
}
