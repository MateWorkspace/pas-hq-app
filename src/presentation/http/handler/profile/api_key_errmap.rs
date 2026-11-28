use crate::domain::models::{ApiKeyError, RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn own_list_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to view your API keys.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "API_KEY_LIST_FAILED",
            "Your API keys could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "API_KEY_LIST_FAILED",
            format!("Your API keys could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn own_create_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to create an API key.",
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
            "You already have an API key with this name.",
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

pub(crate) fn own_rotate_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to rotate an API key.",
        ),
        UsecaseError::Forbidden => {
            ApiError::forbidden("API_KEY_NOT_YOURS", "This API key belongs to another user.")
        }
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

pub(crate) fn own_set_active_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to change an API key.",
        ),
        UsecaseError::Forbidden => {
            ApiError::forbidden("API_KEY_NOT_YOURS", "This API key belongs to another user.")
        }
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

pub(crate) fn own_delete_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to delete an API key.",
        ),
        UsecaseError::Forbidden => {
            ApiError::forbidden("API_KEY_NOT_YOURS", "This API key belongs to another user.")
        }
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
