use crate::domain::models::{PasswordError, RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn create_user_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to create users.",
        ),
        UsecaseError::Validation(ValidatorError::UserNameTooShort) => ApiError::bad_request(
            "USER_NAME_TOO_SHORT",
            "The user's name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UserNameTooLong) => ApiError::bad_request(
            "USER_NAME_TOO_LONG",
            "The user's name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UserBioTooLong) => ApiError::bad_request(
            "USER_BIO_TOO_LONG",
            "The user's bio must be at most 1000 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UsernameTooShort) => ApiError::bad_request(
            "USERNAME_TOO_SHORT",
            "The username must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UsernameTooLong) => ApiError::bad_request(
            "USERNAME_TOO_LONG",
            "The username must be at most 32 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UsernameInvalid) => ApiError::bad_request(
            "USERNAME_INVALID",
            "The username may only contain letters, digits, dots, underscores, and hyphens.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordTooShort) => ApiError::bad_request(
            "PASSWORD_TOO_SHORT",
            "The password must be at least 8 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordTooLong) => ApiError::bad_request(
            "PASSWORD_TOO_LONG",
            "The password must be at most 72 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordMissingUppercase) => {
            ApiError::bad_request(
                "PASSWORD_MISSING_UPPERCASE",
                "The password must contain at least one uppercase letter.",
            )
        }
        UsecaseError::Validation(ValidatorError::PasswordMissingLowercase) => {
            ApiError::bad_request(
                "PASSWORD_MISSING_LOWERCASE",
                "The password must contain at least one lowercase letter.",
            )
        }
        UsecaseError::Validation(ValidatorError::PasswordMissingDigit) => ApiError::bad_request(
            "PASSWORD_MISSING_DIGIT",
            "The password must contain at least one digit.",
        ),
        UsecaseError::Validation(ValidatorError::PasswordMissingSymbol) => ApiError::bad_request(
            "PASSWORD_MISSING_SYMBOL",
            "The password must contain at least one symbol.",
        ),
        UsecaseError::Repository(RepositoryError::UserRoleNotFound) => ApiError::not_found(
            "ROLE_NOT_FOUND",
            "The role you assigned to this user does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::BaseAuthUsernameConflict) => {
            ApiError::conflict("USERNAME_TAKEN", "That username is already in use.")
        }
        UsecaseError::Repository(RepositoryError::BaseAuthEmailConflict) => {
            ApiError::conflict("EMAIL_TAKEN", "That email address is already in use.")
        }
        UsecaseError::Repository(RepositoryError::BaseAuthPhoneConflict) => {
            ApiError::conflict("PHONE_TAKEN", "That phone number is already in use.")
        }
        UsecaseError::Password(PasswordError::HashFailed) => ApiError::internal(
            "USER_CREATE_FAILED",
            "We could not secure the password right now. Please try again.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "USER_CREATE_FAILED",
            "The user could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "USER_CREATE_FAILED",
            format!("The user could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_user_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to update users.",
        ),
        UsecaseError::Validation(ValidatorError::UserNameTooShort) => ApiError::bad_request(
            "USER_NAME_TOO_SHORT",
            "The user's name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UserNameTooLong) => ApiError::bad_request(
            "USER_NAME_TOO_LONG",
            "The user's name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::UserBioTooLong) => ApiError::bad_request(
            "USER_BIO_TOO_LONG",
            "The user's bio must be at most 1000 characters long.",
        ),
        UsecaseError::Repository(RepositoryError::UserNotFound) => ApiError::not_found(
            "USER_NOT_FOUND",
            "The user you tried to update does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::UserRoleNotFound) => ApiError::not_found(
            "ROLE_NOT_FOUND",
            "The role you assigned to this user does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "USER_UPDATE_FAILED",
            "The user could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "USER_UPDATE_FAILED",
            format!("The user could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_user_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to delete users.",
        ),
        UsecaseError::Repository(RepositoryError::UserNotFound) => ApiError::not_found(
            "USER_NOT_FOUND",
            "The user you tried to delete does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "USER_DELETE_FAILED",
            "The user could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "USER_DELETE_FAILED",
            format!("The user could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_users_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view users.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "USER_LIST_FAILED",
            "The users could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "USER_LIST_FAILED",
            format!("The users could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn get_user_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view this user.",
        ),
        UsecaseError::Repository(RepositoryError::UserNotFound) => {
            ApiError::not_found("USER_NOT_FOUND", "The user you requested does not exist.")
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "USER_READ_FAILED",
            "The user could not be read right now. Please try again.",
        ),
        other => ApiError::internal(
            "USER_READ_FAILED",
            format!("The user could not be read. ({})", other.code()),
        ),
    }
}
