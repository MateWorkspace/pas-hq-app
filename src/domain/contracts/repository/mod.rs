pub mod api_key;
pub mod base_auth;
pub mod future;
pub mod inventory;
pub mod location;
pub mod permission;
pub mod price;
pub mod product;
pub mod product_sku;
pub mod product_variant;
pub mod product_variant_selection;
pub mod role;
pub mod role_permission;
pub mod stock_movement;
pub mod user;

pub use api_key::ApiKeyRepository;
pub use base_auth::BaseAuthRepository;
pub use future::RepositoryFuture;
pub use inventory::InventoryRepository;
pub use location::LocationRepository;
pub use permission::PermissionRepository;
pub use price::PriceRepository;
pub use product::ProductRepository;
pub use product_sku::ProductSkuRepository;
pub use product_variant::ProductVariantRepository;
pub use product_variant_selection::ProductVariantSelectionRepository;
pub use role::RoleRepository;
pub use role_permission::RolePermissionRepository;
pub use stock_movement::StockMovementRepository;
pub use user::UserRepository;

/// Offset pagination shared by list operations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}
