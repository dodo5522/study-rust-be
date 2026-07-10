use crate::{
    error_mapper::ErrorMapperTrait,
    models::{histories::ActiveModel, prelude::Histories},
};
use layer_domain::entity::MeasurementEntity;
use layer_use_case::interface::{GenerationError, MeasurementRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseTransaction, entity::EntityTrait};

pub struct MeasurementRepository {}

impl ErrorMapperTrait for MeasurementRepository {}

#[async_trait::async_trait]
impl MeasurementRepositoryTrait<DatabaseTransaction> for MeasurementRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        histories: Vec<MeasurementEntity>,
    ) -> Result<(), GenerationError> {
        let histories = histories
            .into_iter()
            .map(|new| ActiveModel {
                unit: ActiveValue::Set(new.unit.into()),
                group: ActiveValue::Set(new.sub_system),
                label: ActiveValue::Set(new.label),
                value: ActiveValue::Set(new.value),
                monitored_at: ActiveValue::Set(new.monitored_at.into()),
                ..Default::default()
            })
            .collect::<Vec<ActiveModel>>();

        let _ = Histories::insert_many(histories)
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        Ok(())
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        id: i64,
    ) -> Result<Option<MeasurementEntity>, GenerationError> {
        let h = Histories::find_by_id::<i64>(id.into())
            .one(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        if let Some(history) = h {
            Ok(Some(MeasurementEntity {
                value: history.value,
                unit: history
                    .unit
                    .clone()
                    .try_into()
                    .map_err(|_| Self::map_invalid_unit(history.unit))?,
                sub_system: history.group,
                label: history.label,
                monitored_at: history.monitored_at.into(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, tx: &DatabaseTransaction, id: i64) -> Result<(), GenerationError> {
        Err(GenerationError::NotImplemented(
            "HistoryRepository::delete()".to_string(),
        ))
    }
}
