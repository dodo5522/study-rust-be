use dotenvy::var;

/// Get the database URL from environment variables.
///
/// # Arguments
/// - user: A reference to a `String` representing the database user.
/// - password: A reference to a `String` representing the database password.
///
/// # Returns
/// A `String` representing the database URL in the format:
/// `postgresql://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{
pub fn get_db_url(user: &String, password: &String) -> anyhow::Result<String> {
    let db_host = var("DB_HOST")?;
    let db_port = var("DB_PORT")?;
    let db_name = var("DB_NAME")?;
    Ok(format!(
        "postgresql://{user}:{password}@{db_host}:{db_port}/{db_name}"
    ))
}
