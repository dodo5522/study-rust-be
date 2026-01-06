use async_compat::Compat;
use dotenvy::var;
use sea_orm_migration::prelude::cli;
use std::env;

use layer_infra_db::get_db_url;

fn main() -> anyhow::Result<()> {
    let user = var("DB_MIGRATOR_NAME")?;
    let password = var("DB_MIGRATOR_PASSWORD")?;
    let db_url = get_db_url(&user, &password)?;

    env::set_var("DATABASE_URL", db_url);

    Ok(smol::block_on(Compat::new(async {
        cli::run_cli(migration::Migrator).await;
    })))
}
