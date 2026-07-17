use crate::models::sub_systems::{ActiveModel, Model};
use layer_domain::entity::SubSystemEntity;
use sea_orm::ActiveValue;

impl From<SubSystemEntity> for ActiveModel {
    fn from(e: SubSystemEntity) -> Self {
        Self {
            sub_system: ActiveValue::Set(e.sub_system),
            remark: ActiveValue::Set(e.remark),
            ..Default::default()
        }
    }
}

impl From<&SubSystemEntity> for ActiveModel {
    fn from(e: &SubSystemEntity) -> Self {
        Self {
            sub_system: ActiveValue::Set(e.to_owned().sub_system),
            remark: ActiveValue::Set(e.to_owned().remark),
            ..Default::default()
        }
    }
}

impl From<&Model> for SubSystemEntity {
    fn from(m: &Model) -> Self {
        let model = m.to_owned();
        Self {
            sub_system: model.sub_system,
            remark: model.remark,
        }
    }
}

impl From<Model> for SubSystemEntity {
    fn from(m: Model) -> Self {
        Self {
            sub_system: m.sub_system,
            remark: m.remark,
        }
    }
}
