use crate::schemas::GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Measurement {
    Schema,
    Table,
    Id,
    SubSystem,
    Label,
    Unit,
    Value,
    Remark,
    MeasuredAt,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Measurement {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => GENERATION,
                Self::Table => "measurements",
                Self::Id => "id",
                Self::SubSystem => "sub_system",
                Self::Label => "label",
                Self::Unit => "unit",
                Self::Value => "value",
                Self::Remark => "remark",
                Self::MeasuredAt => "measured_at",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}
