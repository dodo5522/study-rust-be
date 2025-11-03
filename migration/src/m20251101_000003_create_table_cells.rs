use sea_orm_migration::{prelude::*, schema::*};

use super::consts::SCHEMA_SPEC;
use super::iden::{cell::Cells, site::Sites};

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Iden for Cells {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_SPEC,
                Self::Table => "cells",
                Self::Id => "id",
                Self::SiteId => "site_id",
                Self::DeviceType => "device_type",
                Self::Frequency => "frequency",
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
                    .table((Cells::Schema, Cells::Table))
                    .if_not_exists()
                    .col(string(Cells::Id))
                    .col(string(Cells::SiteId))
                    .col(string(Cells::DeviceType))
                    .col(float(Cells::Frequency))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cells-site-id")
                            .from((Cells::Schema, Cells::Table), Cells::SiteId)
                            .to((Sites::Schema, Sites::Table), Sites::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .primary_key(
                        Index::create()
                            .col(Cells::Id)
                            .col(Cells::Frequency)
                            .col(Cells::SiteId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Cells::Schema, Cells::Table))
                    .if_exists()
                    .to_owned())
            .await?;

        Ok(())
    }
}
