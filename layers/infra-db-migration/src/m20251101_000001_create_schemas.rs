use sea_orm_migration::prelude::*;
use sea_orm::{Statement, DatabaseBackend};

use super::iden::SCHEMA_SPEC;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                format!("CREATE SCHEMA IF NOT EXISTS {SCHEMA_SPEC};"),
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                format!("DROP SCHEMA IF EXISTS {SCHEMA_SPEC};"),
            ))
            .await?;
        Ok(())
    }
}
