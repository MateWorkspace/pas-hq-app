use crate::domain::models::{RepositoryError, UsecaseError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn create_variant_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to manage product variants.",
        ),
        UsecaseError::Repository(RepositoryError::ProductNotFound) => ApiError::not_found(
            "PRODUCT_NOT_FOUND",
            "The product you are adding a variant to does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantNameConflict) => {
            ApiError::conflict(
                "VARIANT_NAME_TAKEN",
                "This product already has a variant with that name.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "VARIANT_CREATE_FAILED",
            "The variant could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "VARIANT_CREATE_FAILED",
            format!("The variant could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_variant_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to manage product variants.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantNotFound) => ApiError::not_found(
            "VARIANT_NOT_FOUND",
            "The variant you tried to update does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantNameConflict) => {
            ApiError::conflict(
                "VARIANT_NAME_TAKEN",
                "This product already has a variant with that name.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "VARIANT_UPDATE_FAILED",
            "The variant could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "VARIANT_UPDATE_FAILED",
            format!("The variant could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_variant_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to manage product variants.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantNotFound) => ApiError::not_found(
            "VARIANT_NOT_FOUND",
            "The variant you tried to delete does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "VARIANT_DELETE_FAILED",
            "The variant could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "VARIANT_DELETE_FAILED",
            format!("The variant could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_variants_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view product variants.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "VARIANT_LIST_FAILED",
            "The variants could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "VARIANT_LIST_FAILED",
            format!("The variants could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn create_selection_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to manage variant selections.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantNotFound) => ApiError::not_found(
            "VARIANT_NOT_FOUND",
            "The variant you are adding a selection to does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantSelectionNameConflict) => {
            ApiError::conflict(
                "SELECTION_NAME_TAKEN",
                "This variant already has a selection with that name.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "SELECTION_CREATE_FAILED",
            "The selection could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "SELECTION_CREATE_FAILED",
            format!("The selection could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_selection_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to manage variant selections.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantSelectionNotFound) => {
            ApiError::not_found(
                "SELECTION_NOT_FOUND",
                "The selection you tried to update does not exist.",
            )
        }
        UsecaseError::Repository(RepositoryError::ProductVariantSelectionNameConflict) => {
            ApiError::conflict(
                "SELECTION_NAME_TAKEN",
                "This variant already has a selection with that name.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "SELECTION_UPDATE_FAILED",
            "The selection could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "SELECTION_UPDATE_FAILED",
            format!("The selection could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_selection_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to manage variant selections.",
        ),
        UsecaseError::Repository(RepositoryError::ProductVariantSelectionNotFound) => {
            ApiError::not_found(
                "SELECTION_NOT_FOUND",
                "The selection you tried to delete does not exist.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "SELECTION_DELETE_FAILED",
            "The selection could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "SELECTION_DELETE_FAILED",
            format!("The selection could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_selections_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view variant selections.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "SELECTION_LIST_FAILED",
            "The selections could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "SELECTION_LIST_FAILED",
            format!("The selections could not be listed. ({})", other.code()),
        ),
    }
}
