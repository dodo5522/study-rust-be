use crate::models::groups::{ActiveModel, Model};
use layer_domain::entity::SubSystemEntity;
use sea_orm::ActiveValue;

impl From<SubSystemEntity> for ActiveModel {
    fn from(e: SubSystemEntity) -> Self {
        Self {
            group: ActiveValue::Set(e.sub_system),
            remark: ActiveValue::Set(e.remark),
            ..Default::default()
        }
    }
}

impl From<&SubSystemEntity> for ActiveModel {
    fn from(e: &SubSystemEntity) -> Self {
        Self {
            group: ActiveValue::Set(e.to_owned().sub_system),
            remark: ActiveValue::Set(e.to_owned().remark),
            ..Default::default()
        }
    }
}

impl From<&Model> for SubSystemEntity {
    fn from(m: &Model) -> Self {
        let model = m.to_owned();
        Self {
            sub_system: model.group,
            remark: model.remark,
        }
    }
}

impl From<Model> for SubSystemEntity {
    fn from(m: Model) -> Self {
        Self {
            sub_system: m.group,
            remark: m.remark,
        }
    }
}
