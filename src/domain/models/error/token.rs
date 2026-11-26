use std::{error::Error as StdError, fmt};

/// Token generation and validation failures with stable diagnostic codes.
///
/// Expiration is classified separately from an invalid token so callers can react
/// to an expired session without treating a forged token as recoverable. Variants
/// carry no token text or signing material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenError {
    BadArgs,
    Failed,
    Expired,
    Invalid,
}

impl TokenError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::BadArgs => "BAD_ARGS",
            Self::Failed => "TOKEN_FAILED",
            Self::Expired => "TOKEN_EXPIRED",
            Self::Invalid => "TOKEN_INVALID",
        }
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadArgs => formatter.write_str("invalid arguments"),
            Self::Failed => formatter.write_str("token operation failed"),
            Self::Expired => formatter.write_str("token is expired"),
            Self::Invalid => formatter.write_str("token is invalid"),
        }
    }
}

impl StdError for TokenError {}
