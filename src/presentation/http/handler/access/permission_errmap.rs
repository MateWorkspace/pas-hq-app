use crate::domain::models::{RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn create_permission_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to create permissions.",
        ),
        UsecaseError::Validation(ValidatorError::PermissionSlugTooShort) => ApiError::bad_request(
            "PERMISSION_SLUG_TOO_SHORT",
            "The permission slug must be at least 2 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PermissionSlugTooLong) => ApiError::bad_request(
            "PERMISSION_SLUG_TOO_LONG",
            "The permission slug must be at most 64 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PermissionSlugInvalid) => ApiError::bad_request(
            "PERMISSION_SLUG_INVALID",
            "The permission slug may only contain lowercase letters, digits, and single hyphens.",
        ),
        UsecaseError::Validation(ValidatorError::PermissionNameTooShort) => ApiError::bad_request(
            "PERMISSION_NAME_TOO_SHORT",
            "The permission name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PermissionNameTooLong) => ApiError::bad_request(
            "PERMISSION_NAME_TOO_LONG",
            "The permission name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::PermissionDescriptionTooLong) => {
            ApiError::bad_request(
                "PERMISSION_DESCRIPTION_TOO_LONG",
                "The permission description must be at most 500 characters long.",
            )
        }
        UsecaseError::Repository(RepositoryError::PermissionSlugConflict) => ApiError::conflict(
            "PERMISSION_SLUG_TAKEN",
            "Another permission already uses this slug.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PERMISSION_CREATE_FAILED",
            "The permission could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "PERMISSION_CREATE_FAILED",
            format!("The permission could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_permissions_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view permissions.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PERMISSION_LIST_FAILED",
            "The permissions could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "PERMISSION_LIST_FAILED",
            format!("The permissions could not be listed. ({})", other.code()),
        ),
    }
}
