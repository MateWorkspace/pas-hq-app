use crate::domain::models::{RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn create_location_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to create locations.",
        ),
        UsecaseError::Validation(ValidatorError::LocationNameTooShort) => ApiError::bad_request(
            "LOCATION_NAME_TOO_SHORT",
            "The location name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::LocationNameTooLong) => ApiError::bad_request(
            "LOCATION_NAME_TOO_LONG",
            "The location name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::LocationDescriptionTooLong) => {
            ApiError::bad_request(
                "LOCATION_DESCRIPTION_TOO_LONG",
                "The location description must be at most 500 characters long.",
            )
        }
        UsecaseError::Repository(RepositoryError::LocationNameConflict) => ApiError::conflict(
            "LOCATION_NAME_TAKEN",
            "Another location already uses this name.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "LOCATION_CREATE_FAILED",
            "The location could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "LOCATION_CREATE_FAILED",
            format!("The location could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_location_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to update locations.",
        ),
        UsecaseError::Validation(ValidatorError::LocationNameTooShort) => ApiError::bad_request(
            "LOCATION_NAME_TOO_SHORT",
            "The location name must be at least 3 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::LocationNameTooLong) => ApiError::bad_request(
            "LOCATION_NAME_TOO_LONG",
            "The location name must be at most 100 characters long.",
        ),
        UsecaseError::Validation(ValidatorError::LocationDescriptionTooLong) => {
            ApiError::bad_request(
                "LOCATION_DESCRIPTION_TOO_LONG",
                "The location description must be at most 500 characters long.",
            )
        }
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "The location you tried to update does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::LocationNameConflict) => ApiError::conflict(
            "LOCATION_NAME_TAKEN",
            "Another location already uses this name.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "LOCATION_UPDATE_FAILED",
            "The location could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "LOCATION_UPDATE_FAILED",
            format!("The location could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn deactivate_location_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to deactivate locations.",
        ),
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "The location you tried to deactivate does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "LOCATION_DEACTIVATE_FAILED",
            "The location could not be deactivated right now. Please try again.",
        ),
        other => ApiError::internal(
            "LOCATION_DEACTIVATE_FAILED",
            format!("The location could not be deactivated. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_location_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to delete locations.",
        ),
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "The location you tried to delete does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "LOCATION_DELETE_FAILED",
            "The location could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "LOCATION_DELETE_FAILED",
            format!("The location could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_locations_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view locations.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "LOCATION_LIST_FAILED",
            "The locations could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "LOCATION_LIST_FAILED",
            format!("The locations could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn get_location_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view this location.",
        ),
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "The location you requested does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "LOCATION_READ_FAILED",
            "The location could not be read right now. Please try again.",
        ),
        other => ApiError::internal(
            "LOCATION_READ_FAILED",
            format!("The location could not be read. ({})", other.code()),
        ),
    }
}
