use std::{error::Error as StdError, fmt};

/// API key generation and hashing failures with stable diagnostic codes.
///
/// Variants carry no key material, so a failure can be logged and mapped safely.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiKeyError {
    InvalidFormat,
    GenerationFailed,
    HashFailed,
}

impl ApiKeyError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidFormat => "API_KEY_INVALID_FORMAT",
            Self::GenerationFailed => "API_KEY_GENERATION_FAILED",
            Self::HashFailed => "API_KEY_HASH_FAILED",
        }
    }
}

impl fmt::Display for ApiKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => formatter.write_str("api key format is invalid"),
            Self::GenerationFailed => formatter.write_str("failed to generate an api key"),
            Self::HashFailed => formatter.write_str("failed to hash the api key"),
        }
    }
}

impl StdError for ApiKeyError {}
