use sea_orm_migration::prelude::Iden;
use crate::iden::SCHEMA_GENERATION;

pub enum Histories {
    Schema,
    Table,
    Id,
    GroupId,
    SourceId,
    LabelId,
    UnitId,
    Value,
    MonitoredAt,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Histories {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "histories",
                Self::Id => "id",
                Self::GroupId => "group_id",
                Self::SourceId => "source_id",
                Self::LabelId => "label_id",
                Self::UnitId => "unit_id",
                Self::Value => "value",
                Self::MonitoredAt => "monitored_at",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}
