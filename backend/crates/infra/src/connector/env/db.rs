use super::traits::EnvReader;
use crate::errors::InfraError;

#[derive(Debug, Copy, Clone)]
pub enum DatabaseUser {
    Tester,
    Operator,
    Migrator,
}

impl DatabaseUser {
    /// Convert user into the keys of environment variables.
    ///
    /// # Returns
    /// * (Database user's name, Database user's password)
    pub fn as_env_keys(self) -> (&'static str, &'static str) {
        match self {
            Self::Tester => ("DB_TESTER_NAME", "DB_TESTER_PASSWORD"),
            Self::Operator => ("DB_OPERATOR_NAME", "DB_OPERATOR_PASSWORD"),
            Self::Migrator => ("DB_MIGRATOR_NAME", "DB_MIGRATOR_PASSWORD"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatabaseEnvConnector {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user_name: String,
    pub db_user_password: String,
    pub db_pool_max_connections: Option<u32>,
    pub db_pool_min_connections: Option<u32>,
    pub db_pool_connect_timeout_secs: Option<u64>,
    pub db_pool_acquire_timeout_secs: Option<u64>,
    pub db_pool_idle_timeout_secs: Option<u64>,
    pub db_pool_max_lifetime_secs: Option<u64>,
}

impl EnvReader for DatabaseEnvConnector {}
impl DatabaseEnvConnector {
    pub fn new(user: DatabaseUser) -> Result<Self, InfraError> {
        let (key_user_name, key_user_password) = user.as_env_keys();

        Ok(Self {
            db_host: Self::read_env("DB_HOST")?,
            db_port: Self::read_env("DB_PORT")?,
            db_name: Self::read_env("DB_NAME")?,
            db_user_name: Self::read_env(key_user_name)?,
            db_user_password: Self::read_env(key_user_password)?,
            db_pool_max_connections: Self::read_optional_env("DB_POOL_MAX_CONNECTIONS")?,
            db_pool_min_connections: Self::read_optional_env("DB_POOL_MIN_CONNECTIONS")?,
            db_pool_connect_timeout_secs: Self::read_optional_env("DB_POOL_CONNECT_TIMEOUT_SECS")?,
            db_pool_acquire_timeout_secs: Self::read_optional_env("DB_POOL_ACQUIRE_TIMEOUT_SECS")?,
            db_pool_idle_timeout_secs: Self::read_optional_env("DB_POOL_IDLE_TIMEOUT_SECS")?,
            db_pool_max_lifetime_secs: Self::read_optional_env("DB_POOL_MAX_LIFETIME_SECS")?,
        })
    }
}
