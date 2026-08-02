use sea_orm::DbErr;

#[derive(Debug)]
pub enum InfraError {
    EnvIsNotPresent(String),
    EnvIsNotUnicode(String),
    InvalidEnv(String),
    DbFailed(DbErr),
}

impl std::fmt::Display for InfraError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InfraError::EnvIsNotPresent(env_name) => write!(f, "{} is not present", env_name),
            InfraError::EnvIsNotUnicode(env_name) => write!(f, "{} is not unicode", env_name),
            InfraError::InvalidEnv(env_name) => write!(f, "{} is invalid", env_name),
            InfraError::DbFailed(err) => write!(f, "DB error with {}", err),
        }
    }
}

impl std::error::Error for InfraError {}
