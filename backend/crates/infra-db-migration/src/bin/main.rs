use layer_infra::{DatabaseConnector, DatabaseEnvConnector, DatabaseUser};
use sea_orm_migration::prelude::cli;
use std::env::set_var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = DatabaseEnvConnector::new(DatabaseUser::Migrator)?;
    let db_url = DatabaseConnector::new(
        env.db_user_name,
        env.db_user_password,
        env.db_host,
        env.db_port,
        env.db_name,
        None,
    )
    .get_url();

    set_var("DATABASE_URL", db_url);
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
