use std::{
    collections::HashMap,
    env::{self, VarError},
    error::Error,
    fmt,
    fs::File,
    io::{self, Read},
    net::SocketAddr,
    path::Path,
    str::FromStr,
    time::Duration,
};

use crate::domain::models::{
    AppEnv, CacheBackend,
    logger::{Format, Level},
};

const POSTGRES_HOST: &str = "PASHQ_POSTGRES_HOST";
const POSTGRES_PORT: &str = "PASHQ_POSTGRES_PORT";
const POSTGRES_USERNAME: &str = "PASHQ_POSTGRES_USERNAME";
const POSTGRES_PASSWORD: &str = "PASHQ_POSTGRES_PASSWORD";
const POSTGRES_DATABASE: &str = "PASHQ_POSTGRES_DATABASE";
const POSTGRES_SSL_MODE: &str = "PASHQ_POSTGRES_SSL_MODE";
const POSTGRES_MAX_CONNECTIONS: &str = "PASHQ_POSTGRES_MAX_CONNECTIONS";
const POSTGRES_CONNECT_TIMEOUT: &str = "PASHQ_POSTGRES_CONNECT_TIMEOUT_MS";
const CACHE_BACKEND: &str = "PASHQ_CACHE_BACKEND";
const CACHE_MAX_CAPACITY: &str = "PASHQ_CACHE_MAX_CAPACITY";
const CACHE_INDIVIDUAL_TTL: &str = "PASHQ_CACHE_INDIVIDUAL_TTL_MS";
const CACHE_LIST_TTL: &str = "PASHQ_CACHE_LIST_TTL_MS";
const REDIS_URL: &str = "PASHQ_REDIS_URL";
const LOGGER_FORMAT: &str = "PASHQ_LOGGER_FORMAT";
const LOGGER_LEVEL: &str = "PASHQ_LOGGER_LEVEL";
const AUTH_ACCESS_SECRET: &str = "PASHQ_AUTH_ACCESS_SECRET";
const AUTH_ACCESS_TTL: &str = "PASHQ_AUTH_ACCESS_TTL_MS";
const AUTH_REFRESH_TTL: &str = "PASHQ_AUTH_REFRESH_TTL_MS";
const AUTH_BCRYPT_COST: &str = "PASHQ_AUTH_BCRYPT_COST";
const AUTH_API_KEY_CACHE_TTL: &str = "PASHQ_AUTH_API_KEY_CACHE_TTL_MS";
const HTTP_ADDR: &str = "PASHQ_HTTP_ADDR";
const SHUTDOWN_TIMEOUT: &str = "PASHQ_SHUTDOWN_TIMEOUT_MS";

/// Load an optional `.env` from the working directory, overlaid by process variables.
/// Does not mutate the process environment.
pub fn load() -> Result<AppEnv, LoadError> {
    load_from_path(".env")
}

/// Same as `load`, with an explicit optional `.env` path for tooling or deployment.
pub fn load_from_path(path: impl AsRef<Path>) -> Result<AppEnv, LoadError> {
    let values = read_dotenv(path.as_ref())?;
    from_sources(&values, |name| match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(VarError::NotPresent) => Ok(None),
        Err(VarError::NotUnicode(_)) => Err(LoadError::Variable {
            name,
            reason: "must contain valid Unicode",
        }),
    })
}

fn read_dotenv(path: &Path) -> Result<HashMap<String, String>, LoadError> {
    match File::open(path) {
        Ok(file) => parse_dotenv(file),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(HashMap::new()),
        Err(source) => Err(LoadError::DotenvIo(source)),
    }
}

fn parse_dotenv(reader: impl Read) -> Result<HashMap<String, String>, LoadError> {
    dotenvy::from_read_iter(reader)
        .map(|entry| {
            entry.map_err(|error| match error {
                dotenvy::Error::Io(source) => LoadError::DotenvIo(source),
                // Parser errors may carry entire source lines, including secrets.
                _ => LoadError::DotenvSyntax,
            })
        })
        .collect()
}

fn from_sources(
    values: &HashMap<String, String>,
    process: impl Fn(&'static str) -> Result<Option<String>, LoadError>,
) -> Result<AppEnv, LoadError> {
    let get = |name: &'static str, default: Option<&str>| {
        process(name)?
            .or_else(|| values.get(name).cloned())
            .or_else(|| default.map(str::to_owned))
            .ok_or(LoadError::Variable {
                name,
                reason: "must be explicitly supplied",
            })
    };
    let nonblank = |name: &'static str, value: String| {
        if value.trim().is_empty() {
            return Err(LoadError::Variable {
                name,
                reason: "must not be blank",
            });
        }
        Ok(value)
    };
    let positive = |name: &'static str, value: String, reason: &'static str| {
        value
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .ok_or(LoadError::Variable { name, reason })
    };

    let logger_format = get(LOGGER_FORMAT, Some("plain"))?;
    let logger_format = Format::from_str(&logger_format).map_err(|_| LoadError::Variable {
        name: LOGGER_FORMAT,
        reason: "must be plain or json",
    })?;

    let logger_level = get(LOGGER_LEVEL, Some("INFO"))?;
    let logger_level = Level::from_str(&logger_level).map_err(|_| LoadError::Variable {
        name: LOGGER_LEVEL,
        reason: "must be NONE, ERROR, WARN, INFO, or DEBUG",
    })?;

    let postgres_port = get(POSTGRES_PORT, Some("5432"))?
        .parse::<u16>()
        .ok()
        .filter(|port| *port > 0)
        .ok_or(LoadError::Variable {
            name: POSTGRES_PORT,
            reason: "must be an integer from 1 to 65535",
        })?;

    let postgres_ssl_mode = get(POSTGRES_SSL_MODE, Some("prefer"))?;
    if !matches!(
        postgres_ssl_mode.as_str(),
        "disable" | "allow" | "prefer" | "require" | "verify-ca" | "verify-full"
    ) {
        return Err(LoadError::Variable {
            name: POSTGRES_SSL_MODE,
            reason: "must be disable, allow, prefer, require, verify-ca, or verify-full",
        });
    }

    let postgres_max_connections = positive(
        POSTGRES_MAX_CONNECTIONS,
        get(POSTGRES_MAX_CONNECTIONS, Some("10"))?,
        "must be a positive integer",
    )?;
    let postgres_max_connections =
        u32::try_from(postgres_max_connections).map_err(|_| LoadError::Variable {
            name: POSTGRES_MAX_CONNECTIONS,
            reason: "must fit in a 32-bit unsigned integer",
        })?;

    let cache_backend = get(CACHE_BACKEND, Some("moka"))?;
    let cache_backend =
        CacheBackend::from_str(&cache_backend).map_err(|_| LoadError::Variable {
            name: CACHE_BACKEND,
            reason: "must be moka or redis",
        })?;

    let cache_max_capacity = positive(
        CACHE_MAX_CAPACITY,
        get(CACHE_MAX_CAPACITY, Some("10000"))?,
        "must be a positive integer",
    )?;

    // Only required when the Redis backend is selected, so local Moka runs need no URL.
    let redis_url = match cache_backend {
        CacheBackend::Redis => {
            nonblank(REDIS_URL, get(REDIS_URL, Some("redis://127.0.0.1:6379"))?)?
        }
        CacheBackend::Moka => values
            .get(REDIS_URL)
            .cloned()
            .or_else(|| Some("redis://127.0.0.1:6379".to_owned()))
            .unwrap_or_default(),
    };

    let auth_access_secret = nonblank(AUTH_ACCESS_SECRET, get(AUTH_ACCESS_SECRET, None)?)?;

    let auth_bcrypt_cost = positive(
        AUTH_BCRYPT_COST,
        get(AUTH_BCRYPT_COST, Some("12"))?,
        "must be a positive integer",
    )?;
    let auth_bcrypt_cost = u32::try_from(auth_bcrypt_cost).map_err(|_| LoadError::Variable {
        name: AUTH_BCRYPT_COST,
        reason: "must fit in a 32-bit unsigned integer",
    })?;

    let postgres_connect_timeout = duration(
        POSTGRES_CONNECT_TIMEOUT,
        get(POSTGRES_CONNECT_TIMEOUT, Some("10000"))?,
    )?;
    let cache_individual_ttl = duration(
        CACHE_INDIVIDUAL_TTL,
        get(CACHE_INDIVIDUAL_TTL, Some("60000"))?,
    )?;
    let cache_list_ttl = duration(CACHE_LIST_TTL, get(CACHE_LIST_TTL, Some("10000"))?)?;
    let auth_access_ttl = duration(AUTH_ACCESS_TTL, get(AUTH_ACCESS_TTL, Some("900000"))?)?;
    let auth_refresh_ttl = duration(AUTH_REFRESH_TTL, get(AUTH_REFRESH_TTL, Some("86400000"))?)?;
    let auth_api_key_cache_ttl = duration(
        AUTH_API_KEY_CACHE_TTL,
        get(AUTH_API_KEY_CACHE_TTL, Some("60000"))?,
    )?;
    let shutdown_timeout = duration(SHUTDOWN_TIMEOUT, get(SHUTDOWN_TIMEOUT, Some("15000"))?)?;

    let http_addr = address(HTTP_ADDR, get(HTTP_ADDR, Some("0.0.0.0:8080"))?)?;

    Ok(AppEnv {
        logger_format,
        logger_level,
        postgres_host: nonblank(POSTGRES_HOST, get(POSTGRES_HOST, Some("127.0.0.1"))?)?,
        postgres_port,
        postgres_username: nonblank(POSTGRES_USERNAME, get(POSTGRES_USERNAME, None)?)?,
        // Preserve passwords exactly, including intentional whitespace or emptiness.
        postgres_password: get(POSTGRES_PASSWORD, None)?,
        postgres_database: nonblank(POSTGRES_DATABASE, get(POSTGRES_DATABASE, Some("pas_hq"))?)?,
        postgres_ssl_mode,
        postgres_max_connections,
        postgres_connect_timeout,
        cache_backend,
        cache_max_capacity,
        cache_individual_ttl,
        cache_list_ttl,
        redis_url,
        auth_access_secret,
        auth_access_ttl,
        auth_refresh_ttl,
        auth_bcrypt_cost,
        auth_api_key_cache_ttl,
        http_addr,
        shutdown_timeout,
    })
}

fn duration(name: &'static str, value: String) -> Result<Duration, LoadError> {
    let millis = value
        .parse::<u64>()
        .ok()
        .filter(|millis| *millis > 0)
        .ok_or(LoadError::Variable {
            name,
            reason: "must be a positive integer number of milliseconds",
        })?;
    Ok(Duration::from_millis(millis))
}

fn address(name: &'static str, value: String) -> Result<SocketAddr, LoadError> {
    value
        .parse::<SocketAddr>()
        .map_err(|_| LoadError::Variable {
            name,
            reason: "must be a socket address such as 0.0.0.0:8080",
        })
}

/// Configuration errors never carry rejected values or raw `.env` source lines.
#[derive(Debug)]
pub enum LoadError {
    DotenvIo(io::Error),
    DotenvSyntax,
    Variable {
        name: &'static str,
        reason: &'static str,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DotenvIo(_) => formatter.write_str("could not read the environment file"),
            Self::DotenvSyntax => formatter.write_str("environment file contains an invalid entry"),
            Self::Variable { name, reason } => write!(formatter, "{name} {reason}"),
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DotenvIo(source) => Some(source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> HashMap<String, String> {
        HashMap::from([
            (POSTGRES_USERNAME.to_owned(), "user".to_owned()),
            (POSTGRES_PASSWORD.to_owned(), "secret".to_owned()),
            (AUTH_ACCESS_SECRET.to_owned(), "secret".to_owned()),
        ])
    }

    fn parse(values: &HashMap<String, String>) -> Result<AppEnv, LoadError> {
        from_sources(values, |_| Ok(None))
    }

    fn expect_error(values: &HashMap<String, String>) -> LoadError {
        match parse(values) {
            Ok(_) => panic!("configuration should be rejected"),
            Err(error) => error,
        }
    }

    #[test]
    fn defaults_are_typed_and_bounded() {
        let env = parse(&base()).expect("default configuration");

        assert_eq!(env.logger_format, Format::Plain);
        assert_eq!(env.logger_level, Level::Info);
        assert_eq!(env.postgres_port, 5432);
        assert_eq!(env.postgres_ssl_mode, "prefer");
        assert_eq!(env.postgres_max_connections, 10);
        assert_eq!(env.postgres_database, "pas_hq");
        assert_eq!(env.cache_backend, CacheBackend::Moka);
        assert_eq!(env.cache_max_capacity, 10_000);
        assert_eq!(env.auth_bcrypt_cost, 12);
        assert_eq!(env.http_addr, "0.0.0.0:8080".parse().expect("http addr"));
        assert_eq!(env.auth_access_ttl, Duration::from_millis(900_000));
        assert_eq!(env.auth_refresh_ttl, Duration::from_millis(86_400_000));
        assert_eq!(env.cache_individual_ttl, Duration::from_millis(60_000));
        assert_eq!(env.cache_list_ttl, Duration::from_millis(10_000));
        assert_eq!(env.shutdown_timeout, Duration::from_millis(15_000));
    }

    #[test]
    fn process_values_override_file_values() {
        let mut values = base();
        values.insert(POSTGRES_HOST.to_owned(), "file-host".to_owned());

        let env = from_sources(&values, |name| match name {
            POSTGRES_HOST => Ok(Some("process-host".to_owned())),
            _ => Ok(None),
        })
        .expect("overridden configuration");

        assert_eq!(env.postgres_host, "process-host");
    }

    #[test]
    fn a_redis_backend_requires_a_url() {
        let mut values = base();
        values.insert(CACHE_BACKEND.to_owned(), "redis".to_owned());
        values.insert(REDIS_URL.to_owned(), "  ".to_owned());

        assert!(matches!(
            expect_error(&values),
            LoadError::Variable {
                name: REDIS_URL,
                ..
            }
        ));
    }

    #[test]
    fn invalid_values_report_only_names_and_safe_reasons() {
        let mut values = base();
        values.insert(POSTGRES_PORT.to_owned(), "0".to_owned());
        assert!(matches!(
            expect_error(&values),
            LoadError::Variable {
                name: POSTGRES_PORT,
                ..
            }
        ));

        let mut values = base();
        values.insert(CACHE_BACKEND.to_owned(), "memcached".to_owned());
        assert!(matches!(
            expect_error(&values),
            LoadError::Variable {
                name: CACHE_BACKEND,
                ..
            }
        ));

        let mut values = base();
        values.insert(LOGGER_LEVEL.to_owned(), "LOUD".to_owned());
        assert!(matches!(
            expect_error(&values),
            LoadError::Variable {
                name: LOGGER_LEVEL,
                ..
            }
        ));

        let mut values = base();
        values.insert(HTTP_ADDR.to_owned(), "not-an-address".to_owned());
        assert!(matches!(
            expect_error(&values),
            LoadError::Variable {
                name: HTTP_ADDR,
                ..
            }
        ));
    }

    #[test]
    fn a_blank_access_secret_is_rejected() {
        let mut values = base();
        values.insert(AUTH_ACCESS_SECRET.to_owned(), "   ".to_owned());

        assert!(matches!(
            expect_error(&values),
            LoadError::Variable {
                name: AUTH_ACCESS_SECRET,
                ..
            }
        ));
    }

    #[test]
    fn errors_do_not_render_secret_values() {
        let values = HashMap::from([(
            POSTGRES_PASSWORD.to_owned(),
            "super-secret-password".to_owned(),
        )]);
        let error = expect_error(&values);

        assert!(!error.to_string().contains("super-secret-password"));
        assert!(matches!(
            error,
            LoadError::Variable {
                name: AUTH_ACCESS_SECRET,
                ..
            }
        ));
    }
}
