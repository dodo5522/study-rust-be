use sea_orm_migration::{prelude::*, schema::*};

use super::iden::SCHEMA_SPEC;
use super::iden::spec::Devices;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Iden for Devices {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_SPEC,
                Self::Table => "devices",
                Self::Id => "id",
                Self::Type => "type",
                Self::CpuMhz => "cpu_mhz",
                Self::MemoryMb => "memory_mb",
                Self::StorageGb => "storage_gb",
                _ => panic!("Unsupported column"),
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
                    .table((Devices::Schema, Devices::Table))
                    .if_not_exists()
                    .col(string(Devices::Id))
                    .col(string(Devices::Type))
                    .col(float(Devices::CpuMhz))
                    .col(float(Devices::MemoryMb))
                    .col(float(Devices::StorageGb))
                    .primary_key(
                        Index::create()
                            .col(Devices::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Devices::Schema, Devices::Table))
                    .if_exists()
                    .to_owned())
            .await?;

        Ok(())
    }
}
