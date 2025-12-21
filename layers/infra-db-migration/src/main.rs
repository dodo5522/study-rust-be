mod env;

use anyhow::Result;
use sea_orm_migration::prelude::*;
use std::env::set_var;
use async_compat::Compat;

fn main() -> Result<()> {
    set_var("DATABASE_URL", env::get_db_url()?);
    Ok(smol::block_on(Compat::new(async {
        cli::run_cli(migration::Migrator).await;
    })))
}
