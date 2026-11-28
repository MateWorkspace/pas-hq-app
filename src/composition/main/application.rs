use std::sync::Arc;

use crate::{
    application::{
        access::{ApiKeyManagement, PermissionManagement, RoleManagement, UserManagement},
        auth::{ApiKeyAuthentication, SessionUsecase},
        catalog::{PriceManagement, ProductManagement, SkuManagement, VariantManagement},
        inventory::StockManagement,
        location::LocationManagement,
        misc::{AppContextCreator, RolePermissionGuard},
        profile::{AccountManagement, OwnApiKeyManagement, PasswordChange},
    },
    domain::{
        contracts::utility::{ApiKey, Logger, Password, Token},
        models::AppEnv,
        usecases::{
            access::{ManageApiKey, ManagePermission, ManageRole, ManageUser},
            auth::{ApiKeyAuth, Session},
            catalog::{
                ManagePrice, ManageProduct, ManageSku, ManageVariant, ManageVariantSelection,
                RegenerateProductSkus,
            },
            inventory::{AdjustStock, GetStock, ListStock, RecordMovement},
            location::ManageLocation,
            misc::CreateAppContext,
            profile::{ChangePassword, ManageAccount, ManageOwnApiKey},
            shared::PermissionGuard,
        },
    },
    infrastructure::utility::{
        BcryptPassword, JwtToken, PgTransactor, Sha256ApiKey, UuidAppContextFactory,
    },
};

use super::infrastructure::Infrastructure;

/// Every usecase instance presentation depends on.
pub struct Application {
    pub create_app_context: Arc<dyn CreateAppContext>,
    pub session: Arc<dyn Session>,
    pub validate_api_key: Arc<dyn ApiKeyAuth>,

    pub manage_role: Arc<dyn ManageRole>,
    pub manage_permission: Arc<dyn ManagePermission>,
    pub manage_user: Arc<dyn ManageUser>,
    pub manage_api_key: Arc<dyn ManageApiKey>,

    pub manage_account: Arc<dyn ManageAccount>,
    pub change_password: Arc<dyn ChangePassword>,
    pub manage_own_api_key: Arc<dyn ManageOwnApiKey>,

    pub manage_product: Arc<dyn ManageProduct>,
    pub manage_variant: Arc<dyn ManageVariant>,
    pub manage_variant_selection: Arc<dyn ManageVariantSelection>,
    pub manage_sku: Arc<dyn ManageSku>,
    pub regenerate_product_skus: Arc<dyn RegenerateProductSkus>,
    pub manage_price: Arc<dyn ManagePrice>,

    pub get_stock: Arc<dyn GetStock>,
    pub adjust_stock: Arc<dyn AdjustStock>,
    pub list_stock: Arc<dyn ListStock>,
    pub record_movement: Arc<dyn RecordMovement>,

    pub manage_location: Arc<dyn ManageLocation>,
}

pub fn build(infrastructure: Infrastructure, logger: Arc<dyn Logger>, env: &AppEnv) -> Application {
    let transactor = Arc::new(PgTransactor::new(infrastructure.pool.clone()));
    let password: Arc<dyn Password> = Arc::new(BcryptPassword::new(env.auth_bcrypt_cost));
    let token: Arc<dyn Token> = Arc::new(JwtToken::new(
        env.auth_access_secret.clone(),
        env.auth_access_ttl,
        env.auth_refresh_ttl,
    ));
    let api_key_generator: Arc<dyn ApiKey> = Arc::new(Sha256ApiKey::new());

    let permission_guard: Arc<dyn PermissionGuard> = Arc::new(RolePermissionGuard::new(
        infrastructure.permission_guard_cache.clone(),
        logger.clone(),
    ));

    let create_app_context: Arc<dyn CreateAppContext> = Arc::new(AppContextCreator::new(Arc::new(
        UuidAppContextFactory::new(),
    )));

    let session: Arc<dyn Session> = Arc::new(SessionUsecase::new(
        infrastructure.users.clone(),
        infrastructure.base_auths.clone(),
        infrastructure.roles.clone(),
        infrastructure.permission_guard_cache.clone(),
        password.clone(),
        token,
        logger.clone(),
    ));

    let validate_api_key: Arc<dyn ApiKeyAuth> = Arc::new(ApiKeyAuthentication::new(
        infrastructure.api_keys.clone(),
        infrastructure.api_key_cache.clone(),
        api_key_generator.clone(),
        logger.clone(),
    ));

    let manage_role: Arc<dyn ManageRole> = Arc::new(RoleManagement::new(
        infrastructure.roles.clone(),
        infrastructure.permission_guard_cache.clone(),
        infrastructure.permissions.clone(),
        permission_guard.clone(),
        infrastructure.role_validator.clone(),
        logger.clone(),
    ));

    let manage_permission: Arc<dyn ManagePermission> = Arc::new(PermissionManagement::new(
        infrastructure.permissions.clone(),
        permission_guard.clone(),
        infrastructure.permission_validator.clone(),
        logger.clone(),
    ));

    let manage_user: Arc<dyn ManageUser> = Arc::new(UserManagement::new(
        infrastructure.users.clone(),
        infrastructure.base_auths.clone(),
        infrastructure.api_keys.clone(),
        infrastructure.api_key_cache.clone(),
        transactor.clone(),
        permission_guard.clone(),
        infrastructure.auth_validator.clone(),
        infrastructure.user_validator.clone(),
        password.clone(),
        logger.clone(),
    ));

    let manage_api_key: Arc<dyn ManageApiKey> = Arc::new(ApiKeyManagement::new(
        infrastructure.api_keys.clone(),
        infrastructure.api_key_cache.clone(),
        api_key_generator.clone(),
        infrastructure.api_key_validator.clone(),
        permission_guard.clone(),
        logger.clone(),
    ));

    let manage_account: Arc<dyn ManageAccount> = Arc::new(AccountManagement::new(
        infrastructure.users.clone(),
        infrastructure.user_validator.clone(),
        logger.clone(),
    ));

    let change_password: Arc<dyn ChangePassword> = Arc::new(PasswordChange::new(
        infrastructure.base_auths.clone(),
        infrastructure.auth_validator.clone(),
        password,
        logger.clone(),
    ));

    let manage_own_api_key: Arc<dyn ManageOwnApiKey> = Arc::new(OwnApiKeyManagement::new(
        infrastructure.api_keys.clone(),
        infrastructure.api_key_cache.clone(),
        api_key_generator,
        infrastructure.api_key_validator.clone(),
        logger.clone(),
    ));

    let manage_product: Arc<dyn ManageProduct> = Arc::new(ProductManagement::new(
        infrastructure.products.clone(),
        permission_guard.clone(),
        infrastructure.product_validator.clone(),
        logger.clone(),
    ));

    let variants = Arc::new(VariantManagement::new(
        infrastructure.variants.clone(),
        infrastructure.selections.clone(),
        permission_guard.clone(),
        logger.clone(),
    ));
    let manage_variant: Arc<dyn ManageVariant> = variants.clone();
    let manage_variant_selection: Arc<dyn ManageVariantSelection> = variants;

    let skus = Arc::new(SkuManagement::new(
        infrastructure.skus.clone(),
        infrastructure.variants.clone(),
        infrastructure.selections.clone(),
        infrastructure.inventories.clone(),
        transactor.clone(),
        permission_guard.clone(),
        logger.clone(),
    ));
    let manage_sku: Arc<dyn ManageSku> = skus.clone();
    let regenerate_product_skus: Arc<dyn RegenerateProductSkus> = skus;

    let manage_price: Arc<dyn ManagePrice> = Arc::new(PriceManagement::new(
        infrastructure.prices.clone(),
        permission_guard.clone(),
        logger.clone(),
    ));

    let stock = Arc::new(StockManagement::new(
        infrastructure.inventories.clone(),
        infrastructure.movements.clone(),
        transactor,
        permission_guard.clone(),
        logger.clone(),
    ));

    let get_stock: Arc<dyn GetStock> = stock.clone();
    let adjust_stock: Arc<dyn AdjustStock> = stock.clone();
    let list_stock: Arc<dyn ListStock> = stock.clone();
    let record_movement: Arc<dyn RecordMovement> = stock;

    let manage_location: Arc<dyn ManageLocation> = Arc::new(LocationManagement::new(
        infrastructure.locations.clone(),
        permission_guard,
        infrastructure.location_validator.clone(),
        logger,
    ));

    Application {
        create_app_context,
        session,
        validate_api_key,
        manage_role,
        manage_permission,
        manage_user,
        manage_api_key,
        manage_account,
        change_password,
        manage_own_api_key,
        manage_product,
        manage_variant,
        manage_variant_selection,
        manage_sku,
        regenerate_product_skus,
        manage_price,
        get_stock,
        adjust_stock,
        list_stock,
        record_movement,
        manage_location,
    }
}
