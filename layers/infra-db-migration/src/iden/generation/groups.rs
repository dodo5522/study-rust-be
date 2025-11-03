use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_GENERATION;

pub enum Groups {
    Schema,
    Table,
    Id,
    Group,
    CreatedAt,
}

impl Iden for Groups {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "groups",
                Self::Id => "id",
                Self::Group => "group",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
