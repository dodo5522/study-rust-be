use crate::iden::{Label, Measurement, SubSystem, Unit};
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    async fn create_sub_systems_table<'c>(
        &'c self,
        manager: &SchemaManager<'c>,
    ) -> Result<(), DbErr> {
        let table = format!(
            "{}.{}",
            SubSystem::Schema.to_string(),
            SubSystem::Table.to_string()
        );
        manager
            .create_table(
                Table::create()
                    .table((SubSystem::Schema, SubSystem::Table))
                    .if_not_exists()
                    .col(string(SubSystem::SubSystem).primary_key())
                    .col(string(SubSystem::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(SubSystem::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'サブシステム (e.g. Array, Battery, ...)';",
                    table,
                    SubSystem::SubSystem.to_string()
                ),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    r#"
                    CREATE TRIGGER updated_at_setter
                    BEFORE UPDATE ON {}
                    FOR EACH ROW
                    EXECUTE FUNCTION public.set_updated_at();
                    "#,
                    table
                ),
            ))
            .await?;
        Ok(())
    }

    async fn delete_sub_systems_table<'c>(&self, manager: &SchemaManager<'c>) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((SubSystem::Schema, SubSystem::Table))
                    .to_owned(),
            )
            .await
    }

    async fn create_measurements_table<'c>(
        &'c self,
        manager: &SchemaManager<'c>,
    ) -> Result<(), DbErr> {
        let table = format!(
            "{}.{}",
            Measurement::Schema.to_string(),
            Measurement::Table.to_string()
        );
        manager
            .create_table(
                Table::create()
                    .table((Measurement::Schema, Measurement::Table))
                    .if_not_exists()
                    .col(big_integer(Measurement::Id).primary_key().auto_increment())
                    .col(string(Measurement::SubSystem).not_null())
                    .col(string(Measurement::Label).not_null())
                    .col(string(Measurement::Unit).not_null())
                    .col(float(Measurement::Value).not_null())
                    .col(string(Measurement::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Measurement::MeasuredAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Measurement::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Measurement::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurements-group")
                            .from(
                                (Measurement::Schema, Measurement::Table),
                                Measurement::SubSystem,
                            )
                            .to((SubSystem::Schema, SubSystem::Table), SubSystem::SubSystem)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurements-label")
                            .from(
                                (Measurement::Schema, Measurement::Table),
                                Measurement::Label,
                            )
                            .to((Label::Schema, Label::Table), Label::Label)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurements-unit")
                            .from((Measurement::Schema, Measurement::Table), Measurement::Unit)
                            .to((Unit::Schema, Unit::Table), Unit::Unit)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'グループ';",
                    table,
                    Measurement::SubSystem.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'ラベル';",
                    table,
                    Measurement::Label.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '単位';",
                    table,
                    Measurement::Unit.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '値';",
                    table,
                    Measurement::Value.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '計測日時';",
                    table,
                    Measurement::MeasuredAt.to_string()
                ),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    r#"
                    CREATE TRIGGER updated_at_setter
                    BEFORE UPDATE ON {}
                    FOR EACH ROW
                    EXECUTE FUNCTION public.set_updated_at();
                    "#,
                    table
                ),
            ))
            .await?;
        Ok(())
    }

    async fn delete_measurements_table<'c>(
        &'c self,
        manager: &SchemaManager<'c>,
    ) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Measurement::Schema, Measurement::Table))
                    .to_owned(),
            )
            .await
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_sub_systems_table(manager).await?;
        self.create_measurements_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.delete_measurements_table(manager).await?;
        self.delete_sub_systems_table(manager).await
    }
}
