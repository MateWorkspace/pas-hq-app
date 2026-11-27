use std::{error::Error as StdError, fmt};

/// Persistence failures with generic and auth-specific classifications.
///
/// Operational variants retain their native cause; classification variants are
/// safe to map to stable diagnostic codes.
#[derive(Debug)]
pub enum RepositoryError {
    BadArgs,
    NotFound,
    Conflict,
    BadState,
    Timeout {
        source: Box<dyn StdError + Send + Sync + 'static>,
    },
    Failure {
        source: Box<dyn StdError + Send + Sync + 'static>,
    },
    Unimplemented,

    RoleNotFound,
    RoleSlugConflict,

    PermissionNotFound,
    PermissionSlugConflict,

    RolePermissionNotFound,
    RolePermissionConflict,

    UserNotFound,
    UserRoleNotFound,

    BaseAuthNotFound,
    BaseAuthUsernameConflict,
    BaseAuthEmailConflict,
    BaseAuthPhoneConflict,
    BaseAuthUserConflict,

    LocationNotFound,
    LocationNameConflict,

    ProductNotFound,
    ProductBrandModelManufacturerConflict,

    ProductVariantNotFound,
    ProductVariantNameConflict,

    ProductVariantSelectionNotFound,
    ProductVariantSelectionNameConflict,

    ProductSkuNotFound,
    ProductSkuSelectionConflict,
    ProductSkuCodeConflict,

    PriceNotFound,
    PriceConflict,

    InventoryNotFound,
    InventoryConflict,

    StockMovementNotFound,
}

impl RepositoryError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::BadArgs => "BAD_ARGS",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::BadState => "BAD_STATE",
            Self::Timeout { .. } => "TIMEOUT",
            Self::Failure { .. } => "FAILURE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::RoleNotFound => "ROLE_NOT_FOUND",
            Self::RoleSlugConflict => "ROLE_SLUG_CONFLICT",
            Self::PermissionNotFound => "PERMISSION_NOT_FOUND",
            Self::PermissionSlugConflict => "PERMISSION_SLUG_CONFLICT",
            Self::RolePermissionNotFound => "ROLE_PERMISSION_NOT_FOUND",
            Self::RolePermissionConflict => "ROLE_PERMISSION_CONFLICT",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::UserRoleNotFound => "USER_ROLE_NOT_FOUND",
            Self::BaseAuthNotFound => "BASE_AUTH_NOT_FOUND",
            Self::BaseAuthUsernameConflict => "BASE_AUTH_USERNAME_CONFLICT",
            Self::BaseAuthEmailConflict => "BASE_AUTH_EMAIL_CONFLICT",
            Self::BaseAuthPhoneConflict => "BASE_AUTH_PHONE_CONFLICT",
            Self::BaseAuthUserConflict => "BASE_AUTH_USER_CONFLICT",
            Self::LocationNotFound => "LOCATION_NOT_FOUND",
            Self::LocationNameConflict => "LOCATION_NAME_CONFLICT",
            Self::ProductNotFound => "PRODUCT_NOT_FOUND",
            Self::ProductBrandModelManufacturerConflict => {
                "PRODUCT_BRAND_MODEL_MANUFACTURER_CONFLICT"
            }
            Self::ProductVariantNotFound => "PRODUCT_VARIANT_NOT_FOUND",
            Self::ProductVariantNameConflict => "PRODUCT_VARIANT_NAME_CONFLICT",
            Self::ProductVariantSelectionNotFound => "PRODUCT_VARIANT_SELECTION_NOT_FOUND",
            Self::ProductVariantSelectionNameConflict => "PRODUCT_VARIANT_SELECTION_NAME_CONFLICT",
            Self::ProductSkuNotFound => "PRODUCT_SKU_NOT_FOUND",
            Self::ProductSkuSelectionConflict => "PRODUCT_SKU_SELECTION_CONFLICT",
            Self::ProductSkuCodeConflict => "PRODUCT_SKU_CODE_CONFLICT",
            Self::PriceNotFound => "PRICE_NOT_FOUND",
            Self::PriceConflict => "PRICE_CONFLICT",
            Self::InventoryNotFound => "INVENTORY_NOT_FOUND",
            Self::InventoryConflict => "INVENTORY_CONFLICT",
            Self::StockMovementNotFound => "STOCK_MOVEMENT_NOT_FOUND",
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BadArgs => "invalid arguments",
            Self::NotFound => "record was not found",
            Self::Conflict => "record conflicts with existing data",
            Self::BadState => "invalid state",
            Self::Timeout { .. } => "repository operation timed out",
            Self::Failure { .. } => "repository operation failed",
            Self::Unimplemented => "operation is not implemented",
            Self::RoleNotFound => "role was not found",
            Self::RoleSlugConflict => "role slug conflicts with existing data",
            Self::PermissionNotFound => "permission was not found",
            Self::PermissionSlugConflict => "permission slug conflicts with existing data",
            Self::RolePermissionNotFound => "role permission grant was not found",
            Self::RolePermissionConflict => "role permission grant already exists",
            Self::UserNotFound => "user was not found",
            Self::UserRoleNotFound => "user role was not found",
            Self::BaseAuthNotFound => "base authentication record was not found",
            Self::BaseAuthUsernameConflict => "username conflicts with existing data",
            Self::BaseAuthEmailConflict => "email conflicts with existing data",
            Self::BaseAuthPhoneConflict => "phone conflicts with existing data",
            Self::BaseAuthUserConflict => "user already has base authentication data",
            Self::LocationNotFound => "location was not found",
            Self::LocationNameConflict => "location name conflicts with existing data",
            Self::ProductNotFound => "product was not found",
            Self::ProductBrandModelManufacturerConflict => {
                "product brand, model, and manufacturer conflict with existing data"
            }
            Self::ProductVariantNotFound => "product variant was not found",
            Self::ProductVariantNameConflict => "product variant name conflicts with existing data",
            Self::ProductVariantSelectionNotFound => "product variant selection was not found",
            Self::ProductVariantSelectionNameConflict => {
                "product variant selection name conflicts with existing data"
            }
            Self::ProductSkuNotFound => "product SKU was not found",
            Self::ProductSkuSelectionConflict => {
                "product SKU selection combination conflicts with existing data"
            }
            Self::ProductSkuCodeConflict => "product SKU code conflicts with existing data",
            Self::PriceNotFound => "price was not found",
            Self::PriceConflict => "price conflicts with existing data",
            Self::InventoryNotFound => "inventory was not found",
            Self::InventoryConflict => "inventory already exists for this location and SKU",
            Self::StockMovementNotFound => "stock movement was not found",
        })
    }
}

impl StdError for RepositoryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Timeout { source } | Self::Failure { source } => {
                Some(source.as_ref() as &(dyn StdError + 'static))
            }
            _ => None,
        }
    }
}
