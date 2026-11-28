use crate::domain::models::{PasswordError, RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn get_account_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to view your profile.",
        ),
        UsecaseError::Repository(RepositoryError::UserNotFound) => {
            ApiError::not_found("PROFILE_NOT_FOUND", "Your profile could not be found.")
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "PROFILE_READ_FAILED",
            "Your profile could not be read right now. Please try again.",
        ),
        other => ApiError::internal(
            "PROFILE_READ_FAILED",
            format!("Your profile could not be read. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_account_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to update your profile.",
        ),
        UsecaseError::Validation(ValidatorError::UserNameTooShort) => ApiError::bad_request(
            "USER_NAME_TOO_SHORT",
            "Your name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UserNameTooLong) => ApiError::bad_request(
            "USER_NAME_TOO_LONG",
            "Your name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UserBioTooLong) => ApiError::bad_request(
            "USER_BIO_TOO_LONG",
            "Your bio must be at most 1000 characters long.",
        ),
        UsecaseError::Repository(RepositoryError::UserNotFound) => {
            ApiError::not_found("PROFILE_NOT_FOUND", "Your profile could not be found.")
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "PROFILE_UPDATE_FAILED",
            "Your profile could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "PROFILE_UPDATE_FAILED",
            format!("Your profile could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn change_password_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to change your password.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordTooShort) => ApiError::bad_request(
            "PASSWORD_TOO_SHORT",
            "The new password must be at least 8 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordTooLong) => ApiError::bad_request(
            "PASSWORD_TOO_LONG",
            "The new password must be at most 72 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordMissingUppercase) => {
            ApiError::bad_request(
                "PASSWORD_MISSING_UPPERCASE",
                "The new password must contain at least one uppercase letter.",
            )
        }
        UsecaseError::Validation(ValidatorError::PasswordMissingLowercase) => {
            ApiError::bad_request(
                "PASSWORD_MISSING_LOWERCASE",
                "The new password must contain at least one lowercase letter.",
            )
        }
        UsecaseError::Validation(ValidatorError::PasswordMissingDigit) => ApiError::bad_request(
            "PASSWORD_MISSING_DIGIT",
            "The new password must contain at least one digit.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordMissingSymbol) => ApiError::bad_request(
            "PASSWORD_MISSING_SYMBOL",
            "The new password must contain at least one symbol.",
        ),
        UsecaseError::Password(PasswordError::Mismatch) => ApiError::bad_request(
            "CURRENT_PASSWORD_INCORRECT",
            "The current password you entered is incorrect.",
        ),
        UsecaseError::Repository(RepositoryError::BaseAuthNotFound) => ApiError::not_found(
            "CREDENTIALS_NOT_FOUND",
            "Your sign-in credentials could not be found.",
        ),
        UsecaseError::Password(PasswordError::HashFailed) => ApiError::internal(
            "PASSWORD_CHANGE_FAILED",
            "We could not secure the new password right now. Please try again.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PASSWORD_CHANGE_FAILED",
            "Your password could not be changed right now. Please try again.",
        ),
        other => ApiError::internal(
            "PASSWORD_CHANGE_FAILED",
            format!("Your password could not be changed. ({})", other.code()),
        ),
    }
}
