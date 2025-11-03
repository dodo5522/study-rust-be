use sea_orm_migration::{prelude::*, schema::*};

use super::consts::SCHEMA_SPEC;

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
                    .col(string(Cells::DeviceType))
                    .col(float(Cells::Frequency))
                    .primary_key(Index::create().col(Cells::Id))
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

enum Cells {
    Schema,
    Table,
    Id,
    DeviceType,
    Frequency,
}
