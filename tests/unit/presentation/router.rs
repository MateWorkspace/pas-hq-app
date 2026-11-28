use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use pas_hq_app::{
    domain::{
        contracts::utility::Logger,
        models::{AccessClaims, AppContext, TokenError, UsecaseError},
        usecases::{
            UsecaseFuture,
            access::{ManageApiKey, ManagePermission, ManageRole, ManageUser},
            auth::{
                ApiKeyAuth, RefreshInput, Session, SessionProfile, SessionTokens, SessionUser,
                SignInInput,
            },
            catalog::{
                ManagePrice, ManageProduct, ManageSku, ManageVariant, ManageVariantSelection,
                RegenerateProductSkus,
            },
            inventory::{AdjustStock, GetStock, ListStock, RecordMovement},
            location::ManageLocation,
            misc::CreateAppContext,
            profile::{ChangePassword, ManageAccount, ManageOwnApiKey},
        },
    },
    presentation::http::{Dependencies, router},
};
use tower::ServiceExt;

struct StubContext {
    logger: Arc<dyn Logger>,
}

impl CreateAppContext for StubContext {
    fn create<'a>(
        &'a self,
        actor: Option<String>,
        trace_id: Option<uuid::Uuid>,
        claims: Option<AccessClaims>,
    ) -> UsecaseFuture<'a, AppContext> {
        let _ = &self.logger;
        Box::pin(async move {
            Ok(AppContext {
                actor,
                trace_id,
                execution_id: Some(uuid::Uuid::new_v4()),
                claims,
            })
        })
    }

    fn create_open<'a>(&'a self, trace_id: Option<uuid::Uuid>) -> UsecaseFuture<'a, AppContext> {
        Box::pin(async move {
            Ok(AppContext {
                actor: Some("open".into()),
                trace_id,
                execution_id: Some(uuid::Uuid::new_v4()),
                claims: None,
            })
        })
    }

    fn create_system<'a>(&'a self, trace_id: Option<uuid::Uuid>) -> UsecaseFuture<'a, AppContext> {
        Box::pin(async move {
            Ok(AppContext {
                actor: Some("system".into()),
                trace_id,
                execution_id: Some(uuid::Uuid::new_v4()),
                claims: None,
            })
        })
    }

    fn create_authenticated<'a>(
        &'a self,
        claims: AccessClaims,
        trace_id: Option<uuid::Uuid>,
    ) -> UsecaseFuture<'a, AppContext> {
        Box::pin(async move {
            Ok(AppContext {
                actor: Some(claims.name.clone()),
                trace_id,
                execution_id: Some(uuid::Uuid::new_v4()),
                claims: Some(claims),
            })
        })
    }
}

struct StubSession;

impl Session for StubSession {
    fn sign_in<'a>(
        &'a self,
        _context: &'a AppContext,
        _input: SignInInput,
    ) -> UsecaseFuture<'a, SessionTokens> {
        Box::pin(async {
            Err(UsecaseError::Repository(
                pas_hq_app::domain::models::RepositoryError::Unimplemented,
            ))
        })
    }

    fn validate_access<'a>(
        &'a self,
        _context: &'a AppContext,
        access_token: &'a str,
    ) -> UsecaseFuture<'a, AccessClaims> {
        let token = access_token.to_owned();
        Box::pin(async move {
            if token == "good-token" || token == "admin-token" {
                Ok(AccessClaims {
                    user_id: 1,
                    name: "Ada".into(),
                    role_id: 2,
                    role_name: "Administrator".into(),
                    role: "admin".into(),
                })
            } else {
                Err(UsecaseError::Token(TokenError::Invalid))
            }
        })
    }

    fn refresh<'a>(
        &'a self,
        _context: &'a AppContext,
        _input: RefreshInput,
    ) -> UsecaseFuture<'a, SessionTokens> {
        Box::pin(async {
            Err(UsecaseError::Repository(
                pas_hq_app::domain::models::RepositoryError::Unimplemented,
            ))
        })
    }

    fn profile<'a>(&'a self, context: &'a AppContext) -> UsecaseFuture<'a, SessionProfile> {
        let claims = context.claims.clone();
        Box::pin(async move {
            let Some(claims) = claims else {
                return Err(UsecaseError::Unauthorized);
            };

            Ok(SessionProfile {
                user: SessionUser {
                    id: claims.user_id,
                    name: claims.name,
                    role_id: claims.role_id,
                    role_name: claims.role_name,
                    role: claims.role,
                },
                permissions: vec!["product:list".into(), "product:get".into()],
            })
        })
    }
}

macro_rules! stub_usecase {
    ($ty:ident) => {
        struct $ty;
    };
}

stub_usecase!(StubRole);
stub_usecase!(StubPermission);
stub_usecase!(StubUser);
stub_usecase!(StubAccount);
stub_usecase!(StubPassword);
stub_usecase!(StubProduct);
stub_usecase!(StubVariant);
stub_usecase!(StubSelection);
stub_usecase!(StubSku);
stub_usecase!(StubRegenerate);
stub_usecase!(StubPrice);
stub_usecase!(StubStockRead);
stub_usecase!(StubStockAdjust);
stub_usecase!(StubStockList);
stub_usecase!(StubMovement);
stub_usecase!(StubLocation);
stub_usecase!(StubApiKeyAuth);
stub_usecase!(StubManageApiKey);
stub_usecase!(StubManageOwnApiKey);

/// List stubs return an empty page so query parsing is the only variable.
fn empty<'a, T: Default + Send + 'a>() -> UsecaseFuture<'a, T> {
    Box::pin(async { Ok(T::default()) })
}

fn unreachable_repository<'a, T: Send + 'a>() -> UsecaseFuture<'a, T> {
    Box::pin(async {
        Err(UsecaseError::Repository(
            pas_hq_app::domain::models::RepositoryError::Unimplemented,
        ))
    })
}

impl ManageRole for StubRole {
    fn create_role<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::role::CreateRole,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Role> {
        unreachable_repository()
    }
    fn update_role<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::role::UpdateRole,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Role> {
        unreachable_repository()
    }
    fn delete_role<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_roles<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::role::ListRoles,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Role>> {
        empty()
    }
    fn get_role_permissions<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, Vec<String>> {
        unreachable_repository()
    }
    fn grant_permission<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::role::GrantPermission,
    ) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn revoke_permission<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::role::GrantPermission,
    ) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
}

impl ManagePermission for StubPermission {
    fn create_permission<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::permission::CreatePermission,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Permission> {
        unreachable_repository()
    }
    fn list_permissions<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::permission::ListPermissions,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Permission>> {
        empty()
    }
}

impl ManageUser for StubUser {
    fn create_user<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::user::CreateUser,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::User> {
        unreachable_repository()
    }
    fn update_user<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::user::UpdateUser,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::User> {
        unreachable_repository()
    }
    fn delete_user<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_users<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::user::ListUsers,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::User>> {
        empty()
    }
    fn get_user<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::User> {
        unreachable_repository()
    }
}

impl ManageAccount for StubAccount {
    fn get_account<'a>(
        &'a self,
        _: &'a AppContext,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::User> {
        unreachable_repository()
    }
    fn update_account<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::profile::account::UpdateAccount,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::User> {
        unreachable_repository()
    }
}

impl ChangePassword for StubPassword {
    fn change_password<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::profile::password::ChangePasswordInput,
    ) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
}

impl ManageProduct for StubProduct {
    fn create_product<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::product::CreateProduct,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Product> {
        unreachable_repository()
    }
    fn update_product<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::product::UpdateProduct,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Product> {
        unreachable_repository()
    }
    fn delete_product<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_products<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::product::ListProducts,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Product>> {
        empty()
    }
    fn get_product<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Product> {
        unreachable_repository()
    }
}

impl ManageVariant for StubVariant {
    fn create_variant<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::variant::CreateVariant,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ProductVariant> {
        unreachable_repository()
    }
    fn update_variant<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::variant::UpdateVariant,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ProductVariant> {
        unreachable_repository()
    }
    fn delete_variant<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_variants<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::ProductVariant>> {
        unreachable_repository()
    }
}

impl ManageVariantSelection for StubSelection {
    fn create_selection<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::variant::CreateSelection,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ProductVariantSelection> {
        unreachable_repository()
    }
    fn update_selection<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::variant::UpdateSelection,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ProductVariantSelection> {
        unreachable_repository()
    }
    fn delete_selection<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_selections<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::ProductVariantSelection>> {
        unreachable_repository()
    }
}

impl ManageSku for StubSku {
    fn list_skus<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::ProductSku>> {
        unreachable_repository()
    }
    fn set_sku_code<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::sku::SetSkuCode,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ProductSku> {
        unreachable_repository()
    }
    fn delete_sku<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
}

impl RegenerateProductSkus for StubRegenerate {
    fn regenerate_product_skus<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::sku::RegenerateProductSkusInput,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::usecases::catalog::sku::SkuRegenerationPlan> {
        unreachable_repository()
    }
}

impl ManagePrice for StubPrice {
    fn set_price<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::price::SetPrice,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Price> {
        unreachable_repository()
    }
    fn remove_price<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::price::PriceKey,
    ) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_prices<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Price>> {
        unreachable_repository()
    }
    fn get_effective_price<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::catalog::price::EffectivePriceRequest,
    ) -> UsecaseFuture<'a, Option<pas_hq_app::domain::models::entity::Price>> {
        unreachable_repository()
    }
}

impl GetStock for StubStockRead {
    fn get_stock<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::stock::StockKey,
    ) -> UsecaseFuture<'a, Option<pas_hq_app::domain::models::entity::Inventory>> {
        unreachable_repository()
    }
    fn list_stock_by_location<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::stock::StockByLocation,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Inventory>> {
        unreachable_repository()
    }
    fn list_stock_by_sku<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::stock::StockBySku,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Inventory>> {
        empty()
    }
}

impl AdjustStock for StubStockAdjust {
    fn adjust_stock<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::stock::AdjustStockInput,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Inventory> {
        unreachable_repository()
    }
}

impl ListStock for StubStockList {
    fn list_stock_movements_by_sku<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::stock::MovementBySku,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::StockMovement>> {
        empty()
    }
    fn list_stock_movements_by_location<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::stock::MovementByLocation,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::StockMovement>> {
        unreachable_repository()
    }
}

impl RecordMovement for StubMovement {
    fn transfer_stock<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::inventory::movement::TransferStock,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::StockMovement> {
        unreachable_repository()
    }
}

impl ManageLocation for StubLocation {
    fn create_location<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::location::manage::CreateLocation,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Location> {
        unreachable_repository()
    }
    fn update_location<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::location::manage::UpdateLocation,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Location> {
        unreachable_repository()
    }
    fn deactivate_location<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Location> {
        unreachable_repository()
    }
    fn delete_location<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
    fn list_locations<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::location::manage::ListLocations,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::Location>> {
        empty()
    }
    fn get_location<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::Location> {
        unreachable_repository()
    }
}

impl ApiKeyAuth for StubApiKeyAuth {
    fn validate_api_key<'a>(
        &'a self,
        _context: &'a AppContext,
        raw_key: &'a str,
    ) -> UsecaseFuture<'a, AccessClaims> {
        let key = raw_key.to_owned();
        Box::pin(async move {
            if key == "pas-valid" {
                Ok(AccessClaims {
                    user_id: 1,
                    name: "Ada".into(),
                    role_id: 2,
                    role_name: "Administrator".into(),
                    role: "admin".into(),
                })
            } else {
                Err(UsecaseError::Unauthorized)
            }
        })
    }
}

impl ManageApiKey for StubManageApiKey {
    fn list_api_keys<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::Page,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::ApiKey>> {
        unreachable_repository()
    }
    fn list_user_api_keys<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::api_key::ListUserApiKeys,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::ApiKey>> {
        unreachable_repository()
    }
    fn create_api_key<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::api_key::CreateApiKey,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::usecases::auth::api_key::IssuedApiKey> {
        unreachable_repository()
    }
    fn rotate_api_key<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::usecases::auth::api_key::IssuedApiKey> {
        unreachable_repository()
    }
    fn set_api_key_active<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::access::api_key::SetApiKeyActive,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ApiKey> {
        unreachable_repository()
    }
    fn delete_api_key<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
}

impl ManageOwnApiKey for StubManageOwnApiKey {
    fn list_own_api_keys<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::Page,
    ) -> UsecaseFuture<'a, Vec<pas_hq_app::domain::models::entity::ApiKey>> {
        unreachable_repository()
    }
    fn create_own_api_key<'a>(
        &'a self,
        _: &'a AppContext,
        _: pas_hq_app::domain::usecases::profile::api_key::CreateOwnApiKey,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::usecases::auth::api_key::IssuedApiKey> {
        unreachable_repository()
    }
    fn rotate_own_api_key<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::usecases::auth::api_key::IssuedApiKey> {
        unreachable_repository()
    }
    fn set_own_api_key_active<'a>(
        &'a self,
        _: &'a AppContext,
        _: i64,
        _: bool,
    ) -> UsecaseFuture<'a, pas_hq_app::domain::models::entity::ApiKey> {
        unreachable_repository()
    }
    fn delete_own_api_key<'a>(&'a self, _: &'a AppContext, _: i64) -> UsecaseFuture<'a, ()> {
        unreachable_repository()
    }
}

/// Build the test router with fixed build metadata.
pub(crate) fn stub_app() -> axum::Router {
    router(
        stub_dependencies(),
        pas_hq_app::domain::models::AppInfo {
            name: "pas-hq-app",
            version: "0.1.0-test",
        },
    )
}

pub(crate) fn stub_dependencies() -> Dependencies {
    let logger: Arc<dyn Logger> = Arc::new(
        pas_hq_app::infrastructure::utility::logger::WriterLogger::new(
            std::io::sink(),
            pas_hq_app::domain::models::logger::Format::Json,
            pas_hq_app::domain::models::logger::Level::None,
        ),
    );

    Dependencies {
        create_app_context: Arc::new(StubContext { logger }),
        session: Arc::new(StubSession),
        validate_api_key: Arc::new(StubApiKeyAuth),
        manage_api_key: Arc::new(StubManageApiKey),
        manage_own_api_key: Arc::new(StubManageOwnApiKey),
        manage_role: Arc::new(StubRole),
        manage_permission: Arc::new(StubPermission),
        manage_user: Arc::new(StubUser),
        manage_account: Arc::new(StubAccount),
        change_password: Arc::new(StubPassword),
        manage_product: Arc::new(StubProduct),
        manage_variant: Arc::new(StubVariant),
        manage_variant_selection: Arc::new(StubSelection),
        manage_sku: Arc::new(StubSku),
        regenerate_product_skus: Arc::new(StubRegenerate),
        manage_price: Arc::new(StubPrice),
        get_stock: Arc::new(StubStockRead),
        adjust_stock: Arc::new(StubStockAdjust),
        list_stock: Arc::new(StubStockList),
        record_movement: Arc::new(StubMovement),
        manage_location: Arc::new(StubLocation),
    }
}

#[tokio::test]
async fn protected_route_without_token_is_unauthorized() {
    let app = stub_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/products")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn protected_route_with_invalid_token_is_unauthorized() {
    let app = stub_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/products")
                .header("authorization", "Bearer wrong-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn an_api_key_authenticates_a_protected_route() {
    let app = stub_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/products")
                .header("x-api-key", "pas-valid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // The stub usecase lists nothing; reaching the handler proves authentication
    // succeeded, so this must not be 401.
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn an_invalid_api_key_is_unauthorized() {
    let app = stub_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/products")
                .header("x-api-key", "pas-wrong")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn a_bearer_token_takes_precedence_over_an_api_key() {
    let app = stub_app();

    // Bearer is valid and the API key is not: preferring bearer must succeed.
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/products")
                .header("authorization", "Bearer good-token")
                .header("x-api-key", "pas-wrong")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn unknown_field_in_body_is_rejected() {
    let app = stub_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/products")
                .header("authorization", "Bearer good-token")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"brand":"A","model":"B","manufacturer":"C","nope":1}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
