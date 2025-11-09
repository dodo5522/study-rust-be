use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_SPEC;

pub enum SitesDevices {
    Schema,
    Table,
    SiteId,
    DeviceId,
    CreatedAt,
    UpdatedAt,
}

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
