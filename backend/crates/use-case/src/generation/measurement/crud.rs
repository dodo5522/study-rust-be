use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, MeasurementRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::MeasurementEntity;
use std::marker::PhantomData;

pub struct MeasurementUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> ErrorMapperTrait for MeasurementUseCase<Tx, U, F, R>
{
}

impl<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: MeasurementRepositoryTrait<Tx>,
> MeasurementUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn create(self, histories: Vec<MeasurementEntity>) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        match self.repo.add(uow.ref_tx(), histories).await {
            Ok(()) => {
                uow.commit().await.map_err(Self::map_db_err)?;
                Ok(())
            }
            Err(e) => {
                uow.rollback().await.map_err(Self::map_db_err)?;
                Err(e)
            }
        }
    }

    pub async fn get(self, id: i64) -> Result<Option<MeasurementEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let measurement = self.repo.get(uow.ref_tx(), id.into()).await?;

        match measurement {
            Some(history) => Ok(Some(history.into())),
            None => Ok(None),
        }
    }
}
