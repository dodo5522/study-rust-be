use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::sea_orm::{DatabaseBackend, Statement};

use super::iden::spec::Sites;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table((Sites::Schema, Sites::Table))
                    .add_column_if_not_exists(
                        timestamp_with_time_zone(Sites::CreatedAt)
                            .default(Expr::current_timestamp())
                    )
                    .add_column_if_not_exists(
                        timestamp_with_time_zone(Sites::UpdatedAt)
                            .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    DatabaseBackend::Postgres,
                    "DROP TRIGGER IF EXISTS set_updated_at ON spec.sites;",
                ),
            )
            .await?;

        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    DatabaseBackend::Postgres,
                    r#"
                    CREATE TRIGGER set_updated_at
                    BEFORE UPDATE ON spec.sites
                    FOR EACH ROW
                    EXECUTE FUNCTION public.set_updated_at();
                    "#
                ),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    DatabaseBackend::Postgres,
                    "DROP TRIGGER IF EXISTS set_updated_at ON spec.sites;",
                ),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table((Sites::Schema, Sites::Table))
                    .drop_column(Sites::CreatedAt)
                    .drop_column(Sites::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}
