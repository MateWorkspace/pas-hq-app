use crate::domain::models::{RepositoryError, UsecaseError};
use crate::presentation::http::dto::response::error::ApiError;

pub(crate) fn list_skus_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to view product SKUs.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "SKU_LIST_FAILED",
            "The SKUs could not be listed right now. Please try again.",
        ),
        other => ApiError::internal(
            "SKU_LIST_FAILED",
            format!("The SKUs could not be listed. ({})", other.code()),
        ),
    }
}

pub(crate) fn set_sku_code_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to change SKU codes.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuNotFound) => ApiError::not_found(
            "SKU_NOT_FOUND",
            "The SKU you tried to change does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuCodeConflict) => ApiError::conflict(
            "SKU_CODE_TAKEN",
            "Another SKU of this product already uses that code.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "SKU_CODE_UPDATE_FAILED",
            "The SKU code could not be changed right now. Please try again.",
        ),
        other => ApiError::internal(
            "SKU_CODE_UPDATE_FAILED",
            format!("The SKU code could not be changed. ({})", other.code()),
        ),
    }
}

pub(crate) fn delete_sku_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to delete SKUs.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuNotFound) => ApiError::not_found(
            "SKU_NOT_FOUND",
            "The SKU you tried to delete does not exist.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "SKU_DELETE_FAILED",
            "The SKU could not be deleted right now. Please try again.",
        ),
        other => ApiError::internal(
            "SKU_DELETE_FAILED",
            format!("The SKU could not be deleted. ({})", other.code()),
        ),
    }
}

pub(crate) fn regenerate_skus_error(error: UsecaseError) -> ApiError {
    match error {
        UsecaseError::Forbidden => ApiError::forbidden(
            "PERMISSION_DENIED",
            "You do not have permission to regenerate SKUs.",
        ),
        UsecaseError::Repository(RepositoryError::ProductNotFound) => ApiError::not_found(
            "PRODUCT_NOT_FOUND",
            "The product whose SKUs you are regenerating does not exist.",
        ),
        UsecaseError::Repository(RepositoryError::ProductSkuSelectionConflict) => {
            ApiError::conflict(
                "SKU_COMBINATION_CONFLICT",
                "Two SKUs would have the same selection combination. Check the variant selections.",
            )
        }
        UsecaseError::Repository(RepositoryError::ProductSkuCodeConflict) => ApiError::conflict(
            "SKU_CODE_CONFLICT",
            "A generated SKU would reuse an existing SKU code.",
        ),
        UsecaseError::Repository(_) => ApiError::internal(
            "SKU_REGENERATION_FAILED",
            "The SKUs could not be regenerated right now. Please try again.",
        ),
        other => ApiError::internal(
            "SKU_REGENERATION_FAILED",
            format!("The SKUs could not be regenerated. ({})", other.code()),
        ),
    }
}
