use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum SubSystem {
    Schema,
    Table,
    SubSystem,
    Remark,
    CreatedAt,
}

impl Iden for SubSystem {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => GENERATION,
                Self::Table => "sub_systems",
                Self::SubSystem => "sub_system",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
