use sea_orm::ConnectOptions;
use std::time::Duration;

/// Defines the minimum configuration options of a database.
/// Use default values defined in sea-orm if not set.
#[derive(Debug, Clone, Default)]
pub struct DatabasePoolConfig {
    /// maximum number of connections of the pool
    pub max_connections: Option<u32>,
    /// minimum number of connections of the pool
    pub min_connections: Option<u32>,
    /// timeout duration when acquiring a connection
    pub connect_timeout_secs: Option<u64>,
    /// timeout duration when acquiring a connection
    pub acquire_timeout_secs: Option<u64>,
    /// idle duration before closing a connection
    pub idle_timeout_secs: Option<u64>,
    /// maximum lifetime of individual connections
    pub max_lifetime_secs: Option<u64>,
}

impl DatabasePoolConfig {
    /// Get ConnectOptions for sea-orm
    ///
    /// # Arguments
    /// * url: Database URL string
    /// # Returns
    /// * ConnectOptions instance
    pub fn convert_to(&self, url: String) -> ConnectOptions {
        let mut options = ConnectOptions::new(url);

        if let Some(v) = self.max_connections {
            options.max_connections(v);
        }
        if let Some(v) = self.min_connections {
            options.min_connections(v);
        }
        if let Some(v) = self.connect_timeout_secs {
            options.connect_timeout(Duration::from_secs(v));
        }
        if let Some(v) = self.acquire_timeout_secs {
            options.acquire_timeout(Duration::from_secs(v));
        }
        if let Some(v) = self.idle_timeout_secs {
            options.idle_timeout(Duration::from_secs(v));
        }
        if let Some(v) = self.max_lifetime_secs {
            options.max_lifetime(Duration::from_secs(v));
        }

        options
    }
}
