use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_SPEC;

pub enum Devices {
    Schema,
    Table,
    Id,
    Type,
    CpuMhz,
    MemoryMb,
    StorageGb,
    CreatedAt,
    UpdatedAt,
}

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
