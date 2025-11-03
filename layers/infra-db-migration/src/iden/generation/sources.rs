use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_GENERATION;

pub enum Sources {
    Schema,
    Table,
    Id,
    Source,
    CreatedAt,
}

impl Iden for Sources {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "sources",
                Self::Id => "id",
                Self::Source => "source",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
