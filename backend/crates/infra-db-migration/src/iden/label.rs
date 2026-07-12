use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Label {
    Schema,
    Table,
    Label,
    Remark,
    CreatedAt,
}

impl Iden for Label {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => GENERATION,
                Self::Table => "labels",
                Self::Label => "label",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
