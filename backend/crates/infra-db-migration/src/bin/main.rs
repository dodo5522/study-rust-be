use layer_infra::{DatabaseConnector, DatabaseUser, EnvConnector};
use sea_orm_migration::prelude::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = EnvConnector::new(DatabaseUser::Migrator)?;
    let db_url = DatabaseConnector::new(
        env.db_user_name,
        env.db_user_password,
        env.db_host,
        env.db_port,
        env.db_name,
        None,
    )
    .get_url();

    EnvConnector::set_var("DATABASE_URL", db_url);
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
