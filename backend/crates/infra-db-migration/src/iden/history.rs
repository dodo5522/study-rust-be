use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum History {
    Schema,
    Table,
    Id,
    Group,
    Label,
    Unit,
    Value,
    Remark,
    MonitoredAt,
    CreatedAt,
    UpdatedAt,
}

impl Iden for History {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => GENERATION,
                Self::Table => "histories",
                Self::Id => "id",
                Self::Group => "group",
                Self::Label => "label",
                Self::Unit => "unit",
                Self::Value => "value",
                Self::Remark => "remark",
                Self::MonitoredAt => "monitored_at",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}
