use std::{error::Error as StdError, fmt};

use super::{ApiKeyError, PasswordError, RepositoryError, TokenError, ValidatorError};

/// Application failures preserve the classification of the capability that failed.
#[derive(Debug)]
pub enum UsecaseError {
    Validation(ValidatorError),
    Repository(RepositoryError),
    Password(PasswordError),
    Token(TokenError),
    ApiKey(ApiKeyError),
    Forbidden,
    Unauthorized,
    Database(sqlx::Error),
}

impl UsecaseError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Validation(error) => error.code(),
            Self::Repository(error) => error.code(),
            Self::Password(error) => error.code(),
            Self::Token(error) => error.code(),
            Self::ApiKey(error) => error.code(),
            Self::Forbidden => "FORBIDDEN",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Database(_) => "DATABASE_FAILURE",
        }
    }
}

impl fmt::Display for UsecaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(error) => error.fmt(formatter),
            Self::Repository(error) => error.fmt(formatter),
            Self::Password(error) => error.fmt(formatter),
            Self::Token(error) => error.fmt(formatter),
            Self::ApiKey(error) => error.fmt(formatter),
            Self::Forbidden => formatter.write_str("permission is required for this operation"),
            Self::Unauthorized => formatter.write_str("the request is not authenticated"),
            Self::Database(error) => write!(formatter, "database operation failed: {error}"),
        }
    }
}

impl StdError for UsecaseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            Self::Repository(error) => Some(error),
            Self::Password(error) => Some(error),
            Self::Token(error) => Some(error),
            Self::ApiKey(error) => Some(error),
            Self::Database(error) => Some(error),
            Self::Forbidden | Self::Unauthorized => None,
        }
    }
}

impl From<ValidatorError> for UsecaseError {
    fn from(error: ValidatorError) -> Self {
        Self::Validation(error)
    }
}

impl From<RepositoryError> for UsecaseError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}

impl From<PasswordError> for UsecaseError {
    fn from(error: PasswordError) -> Self {
        Self::Password(error)
    }
}

impl From<TokenError> for UsecaseError {
    fn from(error: TokenError) -> Self {
        Self::Token(error)
    }
}

impl From<ApiKeyError> for UsecaseError {
    fn from(error: ApiKeyError) -> Self {
        Self::ApiKey(error)
    }
}

impl From<sqlx::Error> for UsecaseError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
