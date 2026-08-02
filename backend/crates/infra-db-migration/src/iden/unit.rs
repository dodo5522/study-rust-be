use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Unit {
    Schema,
    Table,
    Unit,
    Remark,
    CreatedAt,
}

impl Iden for Unit {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => GENERATION,
                Self::Table => "units",
                Self::Unit => "unit",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
