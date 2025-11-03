use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_GENERATION;

pub enum Labels {
    Schema,
    Table,
    Id,
    Label,
    CreatedAt,
}

impl Iden for Labels {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "labels",
                Self::Id => "id",
                Self::Label => "label",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
