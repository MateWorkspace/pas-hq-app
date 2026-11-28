use crate::domain::models::{ApiKeyError, RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn list_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view API keys.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_LIST_FAILED",
            "The API keys could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_LIST_FAILED",
            format!("The API keys could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_user_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view this user's API keys.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_LIST_FAILED",
            "This user's API keys could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_LIST_FAILED",
            format!(
                "This user's API keys could not be listed. ({})",
                other.code()
            ),
        ),
    }
}

pub(crate) fn create_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to create API keys.",
        ),
        UsecaseError::Validation(ValidatorError::ApiKeyNameTooShort) => ApiError::bad_request(
            "API_KEY_NAME_TOO_SHORT",
            "The API key name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::ApiKeyNameTooLong) => ApiError::bad_request(
            "API_KEY_NAME_TOO_LONG",
            "The API key name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::ApiKeyDescriptionTooLong) => {
            ApiError::bad_request(
                "API_KEY_DESCRIPTION_TOO_LONG",
                "The API key description must be at most 500 characters long.",
            )
        }
        UsecaseError::Repository(RepositoryError::ApiKeyNameConflict) => ApiError::conflict(
            "API_KEY_NAME_TAKEN",
            "This user already has an API key with this name.",
        ),
        UsecaseError::Repository(RepositoryError::UserNotFound) => ApiError::not_found(
            "USER_NOT_FOUND",
            "The user you are creating an API key for does not exist.",
        ),
        UsecaseError::ApiKey(ApiKeyError::GenerationFailed) => ApiError::internal(
            "API_KEY_CREATE_FAILED",
            "We could not generate an API key right now. Please try again.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_CREATE_FAILED",
            "The API key could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_CREATE_FAILED",
            format!("The API key could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn rotate_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to rotate API keys.",
        ),
        UsecaseError::Repository(RepositoryError::ApiKeyNotFound) => ApiError::not_found(
            "API_KEY_NOT_FOUND",
            "The API key you tried to rotate does not exist.",
        ),
        UsecaseError::ApiKey(_) => ApiError::internal(
            "API_KEY_ROTATE_FAILED",
            "We could not generate a replacement key right now. Please try again.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_ROTATE_FAILED",
            "The API key could not be rotated right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_ROTATE_FAILED",
            format!("The API key could not be rotated. ({})", other.code()),
        ),
    }
}

pub(crate) fn set_active_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to change API keys.",
        ),
        UsecaseError::Repository(RepositoryError::ApiKeyNotFound) => ApiError::not_found(
            "API_KEY_NOT_FOUND",
            "The API key you tried to change does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_UPDATE_FAILED",
            "The API key could not be changed right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_UPDATE_FAILED",
            format!("The API key could not be changed. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to delete API keys.",
        ),
        UsecaseError::Repository(RepositoryError::ApiKeyNotFound) => ApiError::not_found(
            "API_KEY_NOT_FOUND",
            "The API key you tried to delete does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_DELETE_FAILED",
            "The API key could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_DELETE_FAILED",
            format!("The API key could not be deleted. ({})", other.code()),
        ),
    }
}
