use crate::domain::models::{RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn adjust_stock_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to adjust stock.",
        ),
        UsecaseError::Validation(ValidatorError::LocationNameInvalid) => ApiError::bad_request(
            "STOCK_QUANTITY_INVALID",
            "The stock adjustment quantity must not be zero.",
        ),
        UsecaseError::Repository(RepositoryError::BadArgs) => ApiError::bad_request(
            "STOCK_ADJUSTMENT_NOT_ALLOWED",
            "This adjustment would drive the stock below zero, which is not allowed.",
        ),
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "The location you are adjusting stock for does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuNotFound) => ApiError::not_found(
            "SKU_NOT_FOUND",
            "The SKU you are adjusting stock for does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "STOCK_ADJUST_FAILED",
            "The stock could not be adjusted right now. Please try again.",
        ),
        other => ApiError::internal(
            "STOCK_ADJUST_FAILED",
            format!("The stock could not be adjusted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_stock_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view stock.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "STOCK_LIST_FAILED",
            "The stock could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "STOCK_LIST_FAILED",
            format!("The stock could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn transfer_stock_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to transfer stock.",
        ),
        UsecaseError::Repository(RepositoryError::BadArgs) => ApiError::bad_request(
            "TRANSFER_NOT_ALLOWED",
            "This transfer is not allowed. Check that the quantity is positive, the source and destination differ, and the source holds enough stock.",
        ),
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "One of the locations in this transfer does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuNotFound) => ApiError::not_found(
            "SKU_NOT_FOUND",
            "The SKU you are transferring does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "TRANSFER_FAILED",
            "The stock could not be transferred right now. Please try again.",
        ),
        other => ApiError::internal(
            "TRANSFER_FAILED",
            format!("The stock could not be transferred. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_movements_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view stock movements.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "MOVEMENT_LIST_FAILED",
            "The stock movements could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "MOVEMENT_LIST_FAILED",
            format!(
                "The stock movements could not be listed. ({})",
                other.code()
            ),
        ),
    }
}
