use sea_orm_migration::{prelude::*, schema::*};

use super::iden::spec::{Devices, Sites, SitesDevices};
use super::iden::SCHEMA_SPEC;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Iden for SitesDevices {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_SPEC,
                Self::Table => "sites_devices",
                Self::SiteId => "site_id",
                Self::DeviceId => "device_id",
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
                    .table((SitesDevices::Schema, SitesDevices::Table))
                    .if_not_exists()
                    .col(string(SitesDevices::SiteId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sites-devices-to-sites-id")
                            .from((SitesDevices::Schema, SitesDevices::Table), SitesDevices::SiteId)
                            .to((Sites::Schema, Sites::Table), Sites::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                    )
                    .col(string(SitesDevices::DeviceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sites-devices-to-devices-id")
                            .from((SitesDevices::Schema, SitesDevices::Table), SitesDevices::DeviceId)
                            .to((Devices::Schema, Devices::Table), Devices::Id)
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
                    .table((SitesDevices::Schema, SitesDevices::Table))
                    .to_owned())
            .await
    }
}
