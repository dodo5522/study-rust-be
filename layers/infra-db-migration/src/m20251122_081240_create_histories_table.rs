use sea_orm_migration::{prelude::*, schema::*};
use crate::iden::generation::{Groups, Labels, Histories, Sources, Units};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Histories::Schema, Histories::Table))
                    .if_not_exists()
                    .col(
                        big_integer(Histories::Id)
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        integer(Histories::GroupId)
                            .not_null()
                    )
                    .col(
                        integer(Histories::SourceId)
                            .not_null()
                    )
                    .col(
                        integer(Histories::LabelId)
                            .not_null()
                    )
                    .col(
                        integer(Histories::UnitId)
                            .not_null()
                    )
                    .col(
                        float(Histories::Value)
                            .not_null()
                    )
                    .col(
                        timestamp_with_time_zone(Histories::MonitoredAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        timestamp_with_time_zone(Histories::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        timestamp_with_time_zone(Histories::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-group-id")
                            .from((Histories::Schema, Histories::Table), Histories::GroupId)
                            .to((Groups::Schema, Groups::Table), Groups::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-source-id")
                            .from((Histories::Schema, Histories::Table), Histories::SourceId)
                            .to((Sources::Schema, Sources::Table), Sources::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-label-id")
                            .from((Histories::Schema, Histories::Table), Histories::LabelId)
                            .to((Labels::Schema, Labels::Table), Labels::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-unit-id")
                            .from((Histories::Schema, Histories::Table), Histories::UnitId)
                            .to((Units::Schema, Units::Table), Units::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Histories::Schema, Histories::Table))
                    .to_owned()
            )
            .await
    }
}
