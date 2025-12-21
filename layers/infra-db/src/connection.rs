use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};
use super::env::get_db_url;

/// Get a database connection.
///
/// # Returns
/// A `DatabaseConnection` instance connected to the database specified in the environment variables.
pub async fn get_connection(user: &String, password: &String) -> Result<DatabaseConnection> {
    Ok(Database::connect(get_db_url(user, password)?).await?)
}
