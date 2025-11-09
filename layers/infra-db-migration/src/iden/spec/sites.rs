use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_SPEC;

pub enum Sites {
    Schema,
    Table,
    Id,
    Name,
    Address,
    Latitude,
    Longitude,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Sites {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_SPEC,
                Self::Table => "sites",
                Self::Id => "id",
                Self::Name => "name",
                Self::Address => "address",
                Self::Longitude => "longitude",
                Self::Latitude => "latitude",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
                _ => panic!("Unsupported column"),
            }
        )
        .unwrap();
    }
}
