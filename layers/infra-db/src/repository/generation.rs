use layer_domain::{
    EnergyRecord, GenerationId, GenerationRepositoryError as Error, IGenerationRepository,
    value_object,
};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

use crate::models::prelude::{Groups, Histories, Labels, Sources, Units};
use crate::models::{groups, histories, labels, sources, units};

pub struct GenerationRepository {
    db: DatabaseConnection,
}

impl GenerationRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Error> {
        Ok(GenerationRepository { db })
    }
}

#[async_trait::async_trait]
impl IGenerationRepository for GenerationRepository {
    async fn add(&self, new: &EnergyRecord) -> Result<EnergyRecord, Error> {
        let unit = units::ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            ..Default::default()
        };
        let sub_system = groups::ActiveModel {
            group: ActiveValue::Set(new.sub_system.to_owned().into()),
            ..Default::default()
        };
        let source = sources::ActiveModel {
            source: ActiveValue::Set(new.energy_source.to_owned().into()),
            ..Default::default()
        };
        let label = labels::ActiveModel {
            label: ActiveValue::Set(new.label.to_owned()),
            ..Default::default()
        };
        let history = histories::ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            group: ActiveValue::Set(new.sub_system.to_owned().into()),
            label: ActiveValue::Set(new.label.to_owned()),
            source: ActiveValue::Set(new.energy_source.to_owned().into()),
            value: ActiveValue::Set(new.value.to_owned()),
            ..Default::default()
        };

        let res = Units::insert(unit)
            .on_conflict_do_nothing()
            .exec(&self.db)
            .await
            .map_err(|e| Error::Infra(format!("connection failed: {}", e)))?;
        println!("unit: {:?}", res);

        let res = Groups::insert(sub_system)
            .on_conflict_do_nothing()
            .exec(&self.db)
            .await
            .map_err(|e| Error::Infra(format!("connection failed: {}", e)))?;
        println!("group(sub system): {:?}", res);

        let res = Sources::insert(source)
            .on_conflict_do_nothing()
            .exec(&self.db)
            .await
            .map_err(|e| Error::Infra(format!("connection failed: {}", e)))?;
        println!("source: {:?}", res);

        let res = Labels::insert(label)
            .on_conflict_do_nothing()
            .exec(&self.db)
            .await
            .map_err(|e| Error::Infra(format!("connection failed: {}", e)))?;
        println!("label: {:?}", res);

        let res = Histories::insert(history)
            .exec(&self.db)
            .await
            .map_err(|e| Error::Infra(format!("insert failed: {}", e)))?;
        println!("inserted history: {:?}", res);

        let last_insert_id = res.last_insert_id;
        let res = Histories::find_by_id(last_insert_id)
            .one(&self.db)
            .await
            .map_err(|e| Error::Infra(format!("find_by_id({}) failed: {}", last_insert_id, e)))?;
        println!("fetched history: {:?}", res);

        let history = res.ok_or(Error::Infra("not history".to_owned()))?;

        Ok(EnergyRecord {
            id: Some(GenerationId(history.id)),
            value: history.value,
            unit: value_object::Unit::new(history.unit)
                .map_err(|e| Error::Infra(format!("{}", e)))?,
            sub_system: value_object::SubSystem::new(history.group)
                .map_err(|e| Error::Infra(format!("{}", e)))?,
            energy_source: value_object::EnergySource::new(history.source)
                .map_err(|e| Error::Infra(format!("{}", e)))?,
            label: history.label,
            monitored_at: history.monitored_at.into(),
        })
    }

    async fn get(&self, id: GenerationId) -> Result<EnergyRecord, Error> {
        Err(Error::NotImplemented)
    }

    async fn delete(&self, id: GenerationId) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}
