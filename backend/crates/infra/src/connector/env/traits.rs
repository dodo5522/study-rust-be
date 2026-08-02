use crate::InfraError;
use std::env::{VarError, var};

pub trait EnvReader {
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
