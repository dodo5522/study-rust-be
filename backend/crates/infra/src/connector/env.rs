use crate::errors::InfraError;
use std::env::{VarError, set_var, var};

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
pub struct EnvConnector {
    pub host_name_to_bind: String,
    pub port_to_bind: u16,
    pub allowed_origins: Vec<String>,
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

impl EnvConnector {
    pub fn new(user: DatabaseUser) -> Result<Self, InfraError> {
        let (key_user_name, key_user_password) = user.as_env_keys();
        let allowed_origins = var("ALLOWED_ORIGINS")
            .map_err(|_| InfraError::EnvIsNotPresent("ALLOWED_ORIGINS".into()))?
            .split(",")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Self {
            host_name_to_bind: EnvConnector::read_env("BIND_ADDR")?,
            port_to_bind: EnvConnector::read_env("BIND_PORT")?,
            allowed_origins,
            db_host: EnvConnector::read_env("DB_HOST")?,
            db_port: EnvConnector::read_env("DB_PORT")?,
            db_name: EnvConnector::read_env("DB_NAME")?,
            db_user_name: EnvConnector::read_env(key_user_name)?,
            db_user_password: EnvConnector::read_env(key_user_password)?,
            db_pool_max_connections: EnvConnector::read_optional_env("DB_POOL_MAX_CONNECTIONS")?,
            db_pool_min_connections: EnvConnector::read_optional_env("DB_POOL_MIN_CONNECTIONS")?,
            db_pool_connect_timeout_secs: EnvConnector::read_optional_env(
                "DB_POOL_CONNECT_TIMEOUT_SECS",
            )?,
            db_pool_acquire_timeout_secs: EnvConnector::read_optional_env(
                "DB_POOL_ACQUIRE_TIMEOUT_SECS",
            )?,
            db_pool_idle_timeout_secs: EnvConnector::read_optional_env(
                "DB_POOL_IDLE_TIMEOUT_SECS",
            )?,
            db_pool_max_lifetime_secs: EnvConnector::read_optional_env(
                "DB_POOL_MAX_LIFETIME_SECS",
            )?,
        })
    }

    /// Set environment variable to the system.
    ///
    /// # Arguments
    /// * key: Environment variable's name to be set
    /// * value: Environment variable to be set
    ///
    pub fn set_var(key: &str, value: String) {
        unsafe {
            set_var(key, value);
        }
    }

    /// Read environment variable and parse it to the specified type.
    ///
    /// # Arguments
    /// * key: Environment variable's name to be read
    ///
    /// # Returns
    /// * Value with the specified type. Error if the value cannot be parsed or not present.
    fn read_env<V>(key: &str) -> Result<V, InfraError>
    where
        V: std::str::FromStr,
        V::Err: std::fmt::Display,
    {
        match var(key) {
            Ok(value) => value
                .parse::<V>()
                .map_err(|e| InfraError::InvalidEnv(format!("{key} ({e})"))),
            Err(VarError::NotPresent) => Err(InfraError::EnvIsNotPresent(key.to_string())),
            Err(VarError::NotUnicode(_)) => Err(InfraError::EnvIsNotUnicode(key.to_string())),
        }
    }

    /// Read environment variable and parse it to the specified type.
    /// Return None if the environment variable is not present, but return an error otherwise.
    ///
    /// # Arguments
    /// * key: Environment variable's name to be read
    ///
    /// # Returns
    /// * Parsed value, or None if the variable is not present.
    fn read_optional_env<V>(key: &str) -> Result<Option<V>, InfraError>
    where
        V: std::str::FromStr,
        V::Err: std::fmt::Display,
    {
        match var(key) {
            Ok(value) => value
                .parse::<V>()
                .map(Some)
                .map_err(|e| InfraError::InvalidEnv(format!("{key} ({e})"))),
            Err(VarError::NotPresent) => Ok(None),
            Err(VarError::NotUnicode(_)) => Err(InfraError::EnvIsNotUnicode(key.to_string())),
        }
    }
}
