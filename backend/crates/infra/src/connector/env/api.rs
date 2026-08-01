use super::traits::EnvReader;
use crate::errors::InfraError;
use std::env::{VarError, var};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiEnvConnector {
    pub host_name_to_bind: String,
    pub port_to_bind: u16,
    pub allowed_origins: Vec<String>,
}

impl EnvReader for ApiEnvConnector {}
impl ApiEnvConnector {
    pub fn new() -> Result<Self, InfraError> {
        let allowed_origins = var("ALLOWED_ORIGINS")
            .map_err(|e| match e {
                VarError::NotPresent => InfraError::EnvIsNotPresent("ALLOWED_ORIGINS".into()),
                VarError::NotUnicode(_) => InfraError::EnvIsNotUnicode("ALLOWED_ORIGINS".into()),
            })?
            .split(",")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Self {
            host_name_to_bind: Self::read_env("BIND_ADDR")?,
            port_to_bind: Self::read_env("BIND_PORT")?,
            allowed_origins,
        })
    }
}
