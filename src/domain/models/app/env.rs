use std::{error::Error, fmt, net::SocketAddr, str::FromStr, time::Duration};

use crate::domain::models::logger::{Format, Level};

/// Selects which cache adapter composition builds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CacheBackend {
    Moka,
    Redis,
}

impl CacheBackend {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Moka => "moka",
            Self::Redis => "redis",
        }
    }
}

impl fmt::Display for CacheBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CacheBackend {
    type Err = ParseCacheBackendError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "moka" => Ok(Self::Moka),
            "redis" => Ok(Self::Redis),
            _ => Err(ParseCacheBackendError),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseCacheBackendError;

impl fmt::Display for ParseCacheBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("cache backend must be moka or redis")
    }
}

impl Error for ParseCacheBackendError {}

/// Runtime configuration shared by composition and infrastructure.
///
/// Secrets are intentionally excluded from `Debug`; do not derive it on this type.
pub struct AppEnv {
    pub logger_format: Format,
    pub logger_level: Level,

    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_username: String,
    pub postgres_password: String,
    pub postgres_database: String,
    pub postgres_ssl_mode: String,
    pub postgres_max_connections: u32,
    pub postgres_connect_timeout: Duration,

    pub cache_backend: CacheBackend,
    pub cache_max_capacity: u64,
    pub cache_individual_ttl: Duration,
    pub cache_list_ttl: Duration,
    pub redis_url: String,

    pub auth_access_secret: String,
    pub auth_access_ttl: Duration,
    pub auth_refresh_ttl: Duration,
    pub auth_bcrypt_cost: u32,
    pub auth_api_key_cache_ttl: Duration,

    pub http_addr: SocketAddr,
    pub shutdown_timeout: Duration,
}
