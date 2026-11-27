use crate::domain::{
    contracts::utility::Logger,
    models::{AppContext, RepositoryError, logger::Meta},
};

pub(crate) enum OperationError {
    Driver(sqlx::Error),
    Domain(RepositoryError),
}

impl From<sqlx::Error> for OperationError {
    fn from(source: sqlx::Error) -> Self {
        Self::Driver(source)
    }
}

impl From<RepositoryError> for OperationError {
    fn from(error: RepositoryError) -> Self {
        Self::Domain(error)
    }
}

pub(crate) fn finish<T>(
    logger: &dyn Logger,
    context: &AppContext,
    tag: &str,
    result: Result<T, OperationError>,
    not_found: fn() -> RepositoryError,
) -> Result<T, RepositoryError> {
    result.map_err(|error| {
        let mut meta = Meta::new();
        let error = match error {
            OperationError::Domain(error) => error,
            OperationError::Driver(source) => {
                if let Some(database) = source.as_database_error() {
                    if let Some(code) = database.code() {
                        meta.insert("sql_state".into(), code.as_ref().into());
                    }
                    if let Some(constraint) = database.constraint() {
                        meta.insert("constraint".into(), constraint.into());
                    }
                }
                map_driver(source, not_found)
            }
        };
        meta.insert("error_code".into(), error.code().into());
        logger.error(context, tag, &error.to_string(), Some(&meta));
        error
    })
}

fn map_driver(source: sqlx::Error, not_found: fn() -> RepositoryError) -> RepositoryError {
    if matches!(source, sqlx::Error::RowNotFound) {
        return not_found();
    }
    if matches!(source, sqlx::Error::PoolTimedOut)
        || matches!(&source, sqlx::Error::Io(error) if error.kind() == std::io::ErrorKind::TimedOut)
    {
        return RepositoryError::Timeout {
            source: Box::new(source),
        };
    }
    if let Some(database) = source.as_database_error()
        && let Some(code) = database.code()
        && let Some(error) = classify_sqlstate(code.as_ref(), database.constraint())
    {
        return error;
    }
    RepositoryError::Failure {
        source: Box::new(source),
    }
}

fn classify_sqlstate(code: &str, constraint: Option<&str>) -> Option<RepositoryError> {
    match code {
        "23505" => Some(match constraint {
            Some("uq_roles_slug") => RepositoryError::RoleSlugConflict,
            // At most one default role: a second one is a plain conflict.
            Some("uq_roles_is_default") => RepositoryError::Conflict,
            Some("uq_permissions_slug") => RepositoryError::PermissionSlugConflict,
            Some("uq_role_permission_role_id_permission_id") => {
                RepositoryError::RolePermissionConflict
            }
            Some("uq_base_auths_user_id") => RepositoryError::BaseAuthUserConflict,
            Some("uq_base_auths_username") => RepositoryError::BaseAuthUsernameConflict,
            Some("uq_base_auths_email") => RepositoryError::BaseAuthEmailConflict,
            Some("uq_base_auths_phone") => RepositoryError::BaseAuthPhoneConflict,
            Some("uq_locations_name") => RepositoryError::LocationNameConflict,
            Some("uq_products_brand_model_manufacturer") => {
                RepositoryError::ProductBrandModelManufacturerConflict
            }
            Some("uq_product_variants_product_id_name") => {
                RepositoryError::ProductVariantNameConflict
            }
            Some("uq_product_variant_selections_product_variant_id_name") => {
                RepositoryError::ProductVariantSelectionNameConflict
            }
            Some("uq_product_skus_product_id_selection_key") => {
                RepositoryError::ProductSkuSelectionConflict
            }
            Some("uq_product_skus_product_id_code") => RepositoryError::ProductSkuCodeConflict,
            Some("uq_product_sku_selections_sku_id_variant_selection_id") => {
                RepositoryError::ProductSkuSelectionConflict
            }
            Some("uq_prices_global" | "uq_prices_location") => RepositoryError::PriceConflict,
            Some("uq_inventories_location_id_sku_id") => RepositoryError::InventoryConflict,
            _ => RepositoryError::Conflict,
        }),
        "23503" => Some(match constraint {
            Some("fk_users_role") => RepositoryError::UserRoleNotFound,
            Some("fk_role_permission_role") => RepositoryError::RoleNotFound,
            Some("fk_role_permission_permission") => RepositoryError::PermissionNotFound,
            Some("fk_base_auths_user") => RepositoryError::UserNotFound,
            Some("fk_product_variants_product") => RepositoryError::ProductNotFound,
            Some("fk_product_variant_selections_variant") => {
                RepositoryError::ProductVariantNotFound
            }
            Some("fk_product_skus_product") => RepositoryError::ProductNotFound,
            Some("fk_product_sku_selections_sku") => RepositoryError::ProductSkuNotFound,
            Some("fk_product_sku_selections_selection") => {
                RepositoryError::ProductVariantSelectionNotFound
            }
            Some("fk_prices_sku") => RepositoryError::ProductSkuNotFound,
            Some("fk_prices_location") => RepositoryError::LocationNotFound,
            Some("fk_inventories_location") => RepositoryError::LocationNotFound,
            Some("fk_inventories_sku") => RepositoryError::ProductSkuNotFound,
            Some("fk_stock_movements_sku") => RepositoryError::ProductSkuNotFound,
            Some("fk_stock_movements_src_location" | "fk_stock_movements_dest_location") => {
                RepositoryError::LocationNotFound
            }
            _ => RepositoryError::Conflict,
        }),
        "23502" | "23514" | "22P02" | "22001" | "22003" | "22023" => Some(RepositoryError::BadArgs),
        _ => None,
    }
}

pub(crate) fn require_affected(
    rows: u64,
    not_found: fn() -> RepositoryError,
) -> Result<(), OperationError> {
    if rows == 0 {
        Err(not_found().into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_known_unique_constraints() {
        assert!(matches!(
            classify_sqlstate("23505", Some("uq_roles_slug")),
            Some(RepositoryError::RoleSlugConflict)
        ));
        assert!(matches!(
            classify_sqlstate("23505", Some("uq_base_auths_username")),
            Some(RepositoryError::BaseAuthUsernameConflict)
        ));
        assert!(matches!(
            classify_sqlstate("23505", Some("uq_role_permission_role_id_permission_id")),
            Some(RepositoryError::RolePermissionConflict)
        ));
        assert!(matches!(
            classify_sqlstate("23505", Some("unknown_index")),
            Some(RepositoryError::Conflict)
        ));
        assert!(matches!(
            classify_sqlstate("23505", Some("uq_products_brand_model_manufacturer")),
            Some(RepositoryError::ProductBrandModelManufacturerConflict)
        ));
        assert!(matches!(
            classify_sqlstate("23505", Some("uq_prices_location")),
            Some(RepositoryError::PriceConflict)
        ));
        assert!(matches!(
            classify_sqlstate("23505", Some("uq_locations_name")),
            Some(RepositoryError::LocationNameConflict)
        ));
    }

    #[test]
    fn classifies_known_foreign_keys() {
        assert!(matches!(
            classify_sqlstate("23503", Some("fk_users_role")),
            Some(RepositoryError::UserRoleNotFound)
        ));
        assert!(matches!(
            classify_sqlstate("23503", Some("fk_role_permission_permission")),
            Some(RepositoryError::PermissionNotFound)
        ));
        assert!(matches!(
            classify_sqlstate("23503", Some("unknown_constraint")),
            Some(RepositoryError::Conflict)
        ));
        assert!(matches!(
            classify_sqlstate("23503", Some("fk_inventories_sku")),
            Some(RepositoryError::ProductSkuNotFound)
        ));
        assert!(matches!(
            classify_sqlstate("23503", Some("fk_prices_location")),
            Some(RepositoryError::LocationNotFound)
        ));
    }

    #[test]
    fn classifies_arguments_and_unknown_states() {
        assert!(matches!(
            classify_sqlstate("22P02", None),
            Some(RepositoryError::BadArgs)
        ));
        assert!(classify_sqlstate("XX000", None).is_none());
    }
}
