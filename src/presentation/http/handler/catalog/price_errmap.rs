use crate::domain::models::{RepositoryError, UsecaseError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn set_price_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to set prices.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuNotFound) => {
            ApiError::not_found("SKU_NOT_FOUND", "The SKU you are pricing does not exist.")
        }
        UsecaseError::Repository(RepositoryError::LocationNotFound) => ApiError::not_found(
            "LOCATION_NOT_FOUND",
            "The location this price applies to does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::PriceConflict) => ApiError::conflict(
            "PRICE_CONFLICT",
            "A price for this SKU, currency, and location already exists but could not be replaced.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRICE_SET_FAILED",
            "The price could not be saved right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRICE_SET_FAILED",
            format!("The price could not be saved. ({})", other.code()),
        ),
    }
}

pub(crate) fn remove_price_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to remove prices.",
        ),
        UsecaseError::Repository(RepositoryError::PriceNotFound) => ApiError::not_found(
            "PRICE_NOT_FOUND",
            "There is no price to remove for this SKU, currency, and location.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRICE_REMOVE_FAILED",
            "The price could not be removed right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRICE_REMOVE_FAILED",
            format!("The price could not be removed. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_prices_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view prices.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRICE_LIST_FAILED",
            "The prices could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRICE_LIST_FAILED",
            format!("The prices could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn effective_price_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view prices.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRICE_READ_FAILED",
            "The price could not be resolved right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRICE_READ_FAILED",
            format!("The price could not be resolved. ({})", other.code()),
        ),
    }
}
