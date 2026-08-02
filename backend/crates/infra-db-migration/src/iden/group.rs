use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Group {
    Schema,
    Table,
    Group,
    Remark,
    CreatedAt,
}

impl Iden for Group {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => GENERATION,
                Self::Table => "groups",
                Self::Group => "group",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
