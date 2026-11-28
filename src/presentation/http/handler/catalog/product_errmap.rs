use crate::domain::models::{RepositoryError, UsecaseError, ValidatorError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn create_product_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to create products.",
        ),
        UsecaseError::Validation(ValidatorError::ProductBrandInvalid) => ApiError::bad_request(
            "PRODUCT_BRAND_REQUIRED",
            "The product brand must be filled in.",
        ),
        UsecaseError::Validation(ValidatorError::ProductModelInvalid) => ApiError::bad_request(
            "PRODUCT_MODEL_REQUIRED",
            "The product model must be filled in.",
        ),
        UsecaseError::Validation(ValidatorError::ProductManufacturerInvalid) => {
            ApiError::bad_request(
                "PRODUCT_MANUFACTURER_REQUIRED",
                "The product manufacturer must be filled in.",
            )
        }
        UsecaseError::Validation(ValidatorError::ProductDescriptionTooLong) => {
            ApiError::bad_request(
                "PRODUCT_DESCRIPTION_TOO_LONG",
                "The product description must be at most 500 characters long.",
            )
        }
        UsecaseError::Repository(RepositoryError::ProductBrandModelManufacturerConflict) => {
            ApiError::conflict(
                "PRODUCT_ALREADY_EXISTS",
                "A product with this brand, model, and manufacturer already exists.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "PRODUCT_CREATE_FAILED",
            "The product could not be created right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRODUCT_CREATE_FAILED",
            format!("The product could not be created. ({})", other.code()),
        ),
    }
}

pub(crate) fn update_product_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to update products.",
        ),
        UsecaseError::Validation(ValidatorError::ProductBrandInvalid) => ApiError::bad_request(
            "PRODUCT_BRAND_REQUIRED",
            "The product brand must be filled in.",
        ),
        UsecaseError::Validation(ValidatorError::ProductModelInvalid) => ApiError::bad_request(
            "PRODUCT_MODEL_REQUIRED",
            "The product model must be filled in.",
        ),
        UsecaseError::Validation(ValidatorError::ProductManufacturerInvalid) => {
            ApiError::bad_request(
                "PRODUCT_MANUFACTURER_REQUIRED",
                "The product manufacturer must be filled in.",
            )
        }
        UsecaseError::Validation(ValidatorError::ProductDescriptionTooLong) => {
            ApiError::bad_request(
                "PRODUCT_DESCRIPTION_TOO_LONG",
                "The product description must be at most 500 characters long.",
            )
        }
        UsecaseError::Repository(RepositoryError::ProductNotFound) => ApiError::not_found(
            "PRODUCT_NOT_FOUND",
            "The product you tried to update does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductBrandModelManufacturerConflict) => {
            ApiError::conflict(
                "PRODUCT_ALREADY_EXISTS",
                "Another product already uses this brand, model, and manufacturer.",
            )
        }
        UsecaseError::Repository(_) => ApiError::internal(
            "PRODUCT_UPDATE_FAILED",
            "The product could not be updated right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRODUCT_UPDATE_FAILED",
            format!("The product could not be updated. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_product_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to delete products.",
        ),
        UsecaseError::Repository(RepositoryError::ProductNotFound) => ApiError::not_found(
            "PRODUCT_NOT_FOUND",
            "The product you tried to delete does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRODUCT_DELETE_FAILED",
            "The product could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRODUCT_DELETE_FAILED",
            format!("The product could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn list_products_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view products.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRODUCT_LIST_FAILED",
            "The products could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRODUCT_LIST_FAILED",
            format!("The products could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn get_product_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view this product.",
        ),
        UsecaseError::Repository(RepositoryError::ProductNotFound) => ApiError::not_found(
            "PRODUCT_NOT_FOUND",
            "The product you requested does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "PRODUCT_READ_FAILED",
            "The product could not be read right now. Please try again.",
        ),
        other => ApiError::internal(
            "PRODUCT_READ_FAILED",
            format!("The product could not be read. ({})", other.code()),
        ),
    }
}
