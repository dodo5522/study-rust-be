use crate::iden::{Group, History, Label, Unit};
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((History::Schema, History::Table))
                    .if_not_exists()
                    .col(big_integer(History::Id).primary_key().auto_increment())
                    .col(string(History::Group).not_null())
                    .col(string(History::Label).not_null())
                    .col(string(History::Unit).not_null())
                    .col(float(History::Value).not_null())
                    .col(string(History::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(History::MonitoredAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(History::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(History::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-group")
                            .from((History::Schema, History::Table), History::Group)
                            .to((Group::Schema, Group::Table), Group::Group)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-label")
                            .from((History::Schema, History::Table), History::Label)
                            .to((Label::Schema, Label::Table), Label::Label)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-unit")
                            .from((History::Schema, History::Table), History::Unit)
                            .to((Unit::Schema, Unit::Table), Unit::Unit)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!(
            "{}.{}",
            History::Schema.to_string(),
            History::Table.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'グループ';",
                    table,
                    History::Group.to_string()
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
                    History::Label.to_string()
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
                    History::Unit.to_string()
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
                    History::Value.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '観測日時';",
                    table,
                    History::MonitoredAt.to_string()
                ),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                r#"
                    CREATE TRIGGER updated_at_setter
                    BEFORE UPDATE ON generation.histories
                    FOR EACH ROW
                    EXECUTE FUNCTION public.set_updated_at();
                    "#,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((History::Schema, History::Table))
                    .to_owned(),
            )
            .await
    }
}
