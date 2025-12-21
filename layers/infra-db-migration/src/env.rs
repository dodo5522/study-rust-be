use anyhow::Result;
use dotenvy::var;

/// Get the database URL from environment variables.
///
/// # Returns
/// A `String` representing the database URL in the format:
/// `postgresql://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{
pub(crate) fn get_db_url() -> Result<String> {
  let db_host = var("DB_HOST")?;
  let db_port= var("DB_PORT")?;
  let db_name= var("DB_NAME")?;
  let db_user = var("DB_MIGRATOR_NAME")?;
  let db_password = var("DB_MIGRATOR_PASSWORD")?;
  Ok(format!("postgresql://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"))
}
