use sea_orm_migration::prelude::*;
use sea_orm::{Statement, DatabaseBackend};

use super::iden::{SCHEMA_GENERATION, SCHEMA_CONSUMPTION};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for schema in [SCHEMA_GENERATION, SCHEMA_CONSUMPTION] {
            manager
                .get_connection()
                .execute(Statement::from_string(
                    DatabaseBackend::Postgres,
                    format!("CREATE SCHEMA IF NOT EXISTS {schema};"),
                ))
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for schema in [SCHEMA_GENERATION, SCHEMA_CONSUMPTION] {
            manager
                .get_connection()
                .execute(Statement::from_string(
                    DatabaseBackend::Postgres,
                    format!("DROP SCHEMA IF EXISTS {schema};"),
                ))
                .await?;
        }
        Ok(())
    }
}
