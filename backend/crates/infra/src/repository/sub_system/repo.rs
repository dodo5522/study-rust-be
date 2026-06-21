use crate::{
    error_mapper::ErrorMapperTrait,
    models::{groups::ActiveModel, prelude::Groups},
};
use layer_domain::entity::SubSystemEntity;
use layer_use_case::interface::{GenerationError, SubSystemRepositoryTrait};
use sea_orm::{DatabaseTransaction, entity::EntityTrait};

pub struct SubSystemRepository {}

impl ErrorMapperTrait for SubSystemRepository {}

#[async_trait::async_trait]
impl SubSystemRepositoryTrait<DatabaseTransaction> for SubSystemRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        new: &SubSystemEntity,
    ) -> Result<(), GenerationError> {
        let group: ActiveModel = new.into();
        let res = Groups::insert(group)
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(())
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        sub_system: Option<impl AsRef<str> + Send>,
    ) -> Result<Vec<SubSystemEntity>, GenerationError> {
        if let Some(sub_system) = sub_system {
            let found = Groups::find_by_id(sub_system.as_ref().to_string())
                .one(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if let Some(syb_system) = found {
                let s: SubSystemEntity = syb_system.into();
                Ok(vec![s])
            } else {
                Ok(vec![])
            }
        } else {
            let founds = Groups::find()
                .all(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            let systems = founds
                .into_iter()
                .map(|g| Ok(g.into()))
                .collect::<Result<Vec<SubSystemEntity>, GenerationError>>()?;
            Ok(systems)
        }
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        e: &SubSystemEntity,
    ) -> Result<SubSystemEntity, GenerationError> {
        let result = Groups::update::<ActiveModel>(e.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.into())
    }

    async fn delete(
        &self,
        tx: &DatabaseTransaction,
        sub_system: impl AsRef<str> + Send,
    ) -> Result<(), GenerationError> {
        let result = Groups::delete_by_id(sub_system.as_ref().to_string())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        if result.rows_affected == 1 {
            Ok(())
        } else {
            Err(GenerationError::NotFound(sub_system.as_ref().to_string()))
        }
    }
}
