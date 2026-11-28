use crate::domain::models::{RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn create_role_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to create roles.",
        ),
        UsecaseError::Validation(ValidatorError::RoleSlugTooShort) => ApiError::bad_request(
            "ROLE_SLUG_TOO_SHORT",
            "The role slug must be at least 2 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::RoleSlugTooLong) => ApiError::bad_request(
            "ROLE_SLUG_TOO_LONG",
            "The role slug must be at most 64 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::RoleSlugInvalid) => ApiError::bad_request(
            "ROLE_SLUG_INVALID",
            "The role slug may only contain lowercase letters, digits, and single hyphens.",
        ),
        UsecaseError::Validation(ValidatorError::RoleNameTooShort) => ApiError::bad_request(
            "ROLE_NAME_TOO_SHORT",
            "The role name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::RoleNameTooLong) => ApiError::bad_request(
            "ROLE_NAME_TOO_LONG",
            "The role name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::RoleDescriptionTooLong) => ApiError::bad_request(
            "ROLE_DESCRIPTION_TOO_LONG",
            "The role description must be at most 500 characters long.",
        ),
        UsecaseError::Repository(RepositoryError::RoleSlugConflict) => {
            ApiError::conflict("ROLE_SLUG_TAKEN", "Another role already uses this slug.")
        }
        UsecaseError::Repository(RepositoryError::Conflict) => {
            ApiError::conflict("ROLE_CONFLICT", "Only one role can be the default role.")
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "ROLE_CREATE_FAILED",
            "The role could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "ROLE_CREATE_FAILED",
            format!("The role could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_role_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to update roles.",
        ),
        UsecaseError::Validation(ValidatorError::RoleNameTooShort) => ApiError::bad_request(
            "ROLE_NAME_TOO_SHORT",
            "The role name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::RoleNameTooLong) => ApiError::bad_request(
            "ROLE_NAME_TOO_LONG",
            "The role name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::RoleDescriptionTooLong) => ApiError::bad_request(
            "ROLE_DESCRIPTION_TOO_LONG",
            "The role description must be at most 500 characters long.",
        ),
        UsecaseError::Repository(RepositoryError::RoleNotFound) => ApiError::not_found(
            "ROLE_NOT_FOUND",
            "The role you tried to update does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::Conflict) => {
            ApiError::conflict("ROLE_CONFLICT", "Only one role can be the default role.")
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "ROLE_UPDATE_FAILED",
            "The role could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "ROLE_UPDATE_FAILED",
            format!("The role could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_role_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to delete roles.",
        ),
        UsecaseError::Repository(RepositoryError::RoleNotFound) => ApiError::not_found(
            "ROLE_NOT_FOUND",
            "The role you tried to delete does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::Conflict | RepositoryError::UserRoleNotFound) => {
            ApiError::conflict(
                "ROLE_IN_USE",
                "This role is still assigned to users or grants and cannot be deleted.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "ROLE_DELETE_FAILED",
            "The role could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "ROLE_DELETE_FAILED",
            format!("The role could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_roles_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view roles.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "ROLE_LIST_FAILED",
            "The roles could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "ROLE_LIST_FAILED",
            format!("The roles could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn get_role_permissions_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view role permissions.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "ROLE_PERMISSIONS_FAILED",
            "The role permissions could not be read right now. Please try again.",
        ),
        other => ApiError::internal(
            "ROLE_PERMISSIONS_FAILED",
            format!("The role permissions could not be read. ({})", other.code()),
        ),
    }
}

pub(crate) fn grant_permission_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to grant permissions to roles.",
        ),
        UsecaseError::Repository(RepositoryError::RolePermissionConflict) => ApiError::conflict(
            "PERMISSION_ALREADY_GRANTED",
            "This role already has that permission.",
        ),
        UsecaseError::Repository(RepositoryError::RoleNotFound) => ApiError::not_found(
            "ROLE_NOT_FOUND",
            "The role you tried to grant a permission to does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::PermissionNotFound) => ApiError::not_found(
            "PERMISSION_NOT_FOUND",
            "The permission you tried to grant does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PERMISSION_GRANT_FAILED",
            "The permission could not be granted right now. Please try again.",
        ),
        other => ApiError::internal(
            "PERMISSION_GRANT_FAILED",
            format!("The permission could not be granted. ({})", other.code()),
        ),
    }
}

pub(crate) fn revoke_permission_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to revoke permissions from roles.",
        ),
        UsecaseError::Repository(RepositoryError::RolePermissionNotFound) => ApiError::not_found(
            "PERMISSION_GRANT_NOT_FOUND",
            "This role does not currently have that permission.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PERMISSION_REVOKE_FAILED",
            "The permission could not be revoked right now. Please try again.",
        ),
        other => ApiError::internal(
            "PERMISSION_REVOKE_FAILED",
            format!("The permission could not be revoked. ({})", other.code()),
        ),
    }
}
