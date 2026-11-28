use std::{error::Error, fmt, sync::Arc};

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            caching::{ApiKeyClaimsCache, Cache},
            repository::{
                ApiKeyRepository, BaseAuthRepository, InventoryRepository, LocationRepository,
                PermissionRepository, PriceRepository, ProductRepository, ProductSkuRepository,
                ProductVariantRepository, ProductVariantSelectionRepository,
                RolePermissionRepository, RoleRepository, StockMovementRepository, UserRepository,
            },
            validator::{
                ApiKeyValidator, AuthValidator, LocationValidator, PermissionValidator,
                ProductValidator, RoleValidator, UserValidator,
            },
        },
        models::AppEnv,
    },
    infrastructure::{
        caching::{CachedApiKeyClaims, MokaCache, RedisCache},
        repository::{
            CachedRolePermission, PostgresApiKey, PostgresBaseAuth, PostgresInventory,
            PostgresLocation, PostgresPermission, PostgresPrice, PostgresProduct,
            PostgresProductSku, PostgresProductVariant, PostgresProductVariantSelection,
            PostgresRole, PostgresRolePermission, PostgresStockMovement, PostgresUser,
        },
        utility::validator::{
            RegexApiKeyValidator, RegexAuthValidator, RegexLocationValidator,
            RegexPermissionValidator, RegexProductValidator, RegexRoleValidator,
            RegexUserValidator,
        },
    },
};

use super::driver::Drivers;

/// Every concrete dependency the application layer consumes.
pub struct Infrastructure {
    pub pool: PgPool,
    pub permission_guard_cache: Arc<dyn RolePermissionRepository>,
    pub api_key_cache: Arc<dyn ApiKeyClaimsCache>,

    pub roles: Arc<dyn RoleRepository>,
    pub permissions: Arc<dyn PermissionRepository>,
    pub users: Arc<dyn UserRepository>,
    pub base_auths: Arc<dyn BaseAuthRepository>,
    pub api_keys: Arc<dyn ApiKeyRepository>,

    pub products: Arc<dyn ProductRepository>,
    pub variants: Arc<dyn ProductVariantRepository>,
    pub selections: Arc<dyn ProductVariantSelectionRepository>,
    pub skus: Arc<dyn ProductSkuRepository>,
    pub prices: Arc<dyn PriceRepository>,
    pub inventories: Arc<dyn InventoryRepository>,
    pub movements: Arc<dyn StockMovementRepository>,
    pub locations: Arc<dyn LocationRepository>,

    pub api_key_validator: Arc<dyn ApiKeyValidator>,
    pub auth_validator: Arc<dyn AuthValidator>,
    pub location_validator: Arc<dyn LocationValidator>,
    pub permission_validator: Arc<dyn PermissionValidator>,
    pub product_validator: Arc<dyn ProductValidator>,
    pub role_validator: Arc<dyn RoleValidator>,
    pub user_validator: Arc<dyn UserValidator>,
}

pub async fn build(drivers: &Drivers, env: &AppEnv) -> Result<Infrastructure, InfrastructureError> {
    let logger = drivers.logger.clone();
    let pool: PgPool = drivers.pool.clone();

    let cache = cache_backend(env, logger.clone()).await?;

    // Auth and identity
    let roles: Arc<dyn RoleRepository> = Arc::new(PostgresRole::new(pool.clone(), logger.clone()));
    let permissions: Arc<dyn PermissionRepository> =
        Arc::new(PostgresPermission::new(pool.clone(), logger.clone()));
    let users: Arc<dyn UserRepository> = Arc::new(PostgresUser::new(pool.clone(), logger.clone()));
    let base_auths: Arc<dyn BaseAuthRepository> =
        Arc::new(PostgresBaseAuth::new(pool.clone(), logger.clone()));
    let api_keys: Arc<dyn ApiKeyRepository> =
        Arc::new(PostgresApiKey::new(pool.clone(), logger.clone()));

    let role_permissions: Arc<dyn RolePermissionRepository> = Arc::new(CachedRolePermission::new(
        Arc::new(PostgresRolePermission::new(pool.clone(), logger.clone())),
        cache.clone(),
    ));

    let api_key_cache: Arc<dyn ApiKeyClaimsCache> = Arc::new(CachedApiKeyClaims::new(
        cache,
        logger.clone(),
        env.auth_api_key_cache_ttl,
    ));

    // Catalog
    let products: Arc<dyn ProductRepository> =
        Arc::new(PostgresProduct::new(pool.clone(), logger.clone()));
    let variants: Arc<dyn ProductVariantRepository> =
        Arc::new(PostgresProductVariant::new(pool.clone(), logger.clone()));
    let selections: Arc<dyn ProductVariantSelectionRepository> = Arc::new(
        PostgresProductVariantSelection::new(pool.clone(), logger.clone()),
    );
    let skus: Arc<dyn ProductSkuRepository> =
        Arc::new(PostgresProductSku::new(pool.clone(), logger.clone()));
    let prices: Arc<dyn PriceRepository> =
        Arc::new(PostgresPrice::new(pool.clone(), logger.clone()));

    // Inventory and sites
    let inventories: Arc<dyn InventoryRepository> =
        Arc::new(PostgresInventory::new(pool.clone(), logger.clone()));
    let movements: Arc<dyn StockMovementRepository> =
        Arc::new(PostgresStockMovement::new(pool.clone(), logger.clone()));
    let locations: Arc<dyn LocationRepository> =
        Arc::new(PostgresLocation::new(pool.clone(), logger.clone()));

    Ok(Infrastructure {
        pool,
        permission_guard_cache: role_permissions,
        api_key_cache,
        roles,
        permissions,
        users,
        base_auths,
        api_keys,
        products,
        variants,
        selections,
        skus,
        prices,
        inventories,
        movements,
        locations,
        api_key_validator: Arc::new(RegexApiKeyValidator::new(logger.clone())),
        auth_validator: Arc::new(RegexAuthValidator::new(logger.clone())),
        location_validator: Arc::new(RegexLocationValidator::new(logger.clone())),
        permission_validator: Arc::new(RegexPermissionValidator::new(logger.clone())),
        product_validator: Arc::new(RegexProductValidator::new(logger.clone())),
        role_validator: Arc::new(RegexRoleValidator::new(logger.clone())),
        user_validator: Arc::new(RegexUserValidator::new(logger)),
    })
}

async fn cache_backend(
    env: &AppEnv,
    logger: Arc<dyn crate::domain::contracts::utility::Logger>,
) -> Result<Arc<dyn Cache>, InfrastructureError> {
    match env.cache_backend {
        crate::domain::models::CacheBackend::Moka => Ok(Arc::new(MokaCache::new(
            env.cache_max_capacity,
            env.cache_individual_ttl,
            env.cache_list_ttl,
        ))),
        crate::domain::models::CacheBackend::Redis => Ok(Arc::new(
            RedisCache::new(
                &env.redis_url,
                env.cache_individual_ttl,
                env.cache_list_ttl,
                logger,
            )
            .await
            .map_err(InfrastructureError::Cache)?,
        )),
    }
}

#[derive(Debug)]
pub enum InfrastructureError {
    Cache(redis::RedisError),
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cache(_) => formatter.write_str("could not initialize the Redis cache"),
        }
    }
}

impl Error for InfrastructureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Cache(source) => Some(source),
        }
    }
}
