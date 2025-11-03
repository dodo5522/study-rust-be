use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_GENERATION;

pub enum Units {
    Schema,
    Table,
    Id,
    Unit,
    CreatedAt,
}

impl Iden for Units {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "units",
                Self::Id => "id",
                Self::Unit => "unit",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
