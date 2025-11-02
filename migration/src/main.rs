use anyhow::Result;
use sea_orm_migration::prelude::*;
use std::env::set_var;

use layer_infrastructure_db::env::get_db_url;

#[async_std::main]
async fn main() -> Result<()> {
    set_var("DATABASE_URL", get_db_url()?);
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
