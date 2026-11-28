use crate::domain::models::{PasswordError, TokenError, UsecaseError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn sign_in_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Password(PasswordError::Mismatch) => ApiError::unauthorized(
            "INVALID_CREDENTIALS",
            "The username or password you entered is incorrect.",
        ),
        UsecaseError::Password(PasswordError::CompareFailed) => ApiError::internal(
            "SIGN_IN_FAILED",
            "We could not check your password right now. Please try again.",
        ),
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "ACCOUNT_UNAVAILABLE",
            "This account is inactive or suspended and cannot sign in.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "SIGN_IN_FAILED",
            "We could not sign you in right now. Please try again.",
        ),
        UsecaseError::Token(_) => ApiError::internal(
            "SIGN_IN_FAILED",
            "We could not create your session. Please try again.",
        ),
        other => ApiError::internal(
            "SIGN_IN_FAILED",
            format!("We could not sign you in right now. ({})", other.code()),
        ),
    }
}

pub(crate) fn refresh_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Token(TokenError::Expired) => ApiError::unauthorized(
            "REFRESH_TOKEN_EXPIRED",
            "Your session has expired. Please sign in again.",
        ),
        UsecaseError::Token(_) => ApiError::unauthorized(
            "REFRESH_TOKEN_INVALID",
            "Your session is no longer valid. Please sign in again.",
        ),
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "ACCOUNT_UNAVAILABLE",
            "This account is inactive or suspended and cannot continue.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "REFRESH_FAILED",
            "We could not refresh your session right now. Please try again.",
        ),
        other => ApiError::internal(
            "REFRESH_FAILED",
            format!("We could not refresh your session. ({})", other.code()),
        ),
    }
}
