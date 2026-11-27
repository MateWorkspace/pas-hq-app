use pas_hq_app::domain::contracts::repository::{
    BaseAuthRepository, InventoryRepository, LocationRepository, PermissionRepository,
    PriceRepository, ProductRepository, ProductSkuRepository, ProductVariantRepository,
    ProductVariantSelectionRepository, RolePermissionRepository, RoleRepository,
    StockMovementRepository, UserRepository,
};

/// The repository contracts must stay usable through trait objects because the
/// composition root stores them as `Arc<dyn ...>`. This fails to compile if a
/// contract stops being object-safe.
#[test]
fn repository_contracts_are_object_safe() {
    let _: Option<&dyn RoleRepository> = None;
    let _: Option<&dyn PermissionRepository> = None;
    let _: Option<&dyn RolePermissionRepository> = None;
    let _: Option<&dyn UserRepository> = None;
    let _: Option<&dyn BaseAuthRepository> = None;
    let _: Option<&dyn LocationRepository> = None;
    let _: Option<&dyn ProductRepository> = None;
    let _: Option<&dyn ProductVariantRepository> = None;
    let _: Option<&dyn ProductVariantSelectionRepository> = None;
    let _: Option<&dyn ProductSkuRepository> = None;
    let _: Option<&dyn PriceRepository> = None;
    let _: Option<&dyn InventoryRepository> = None;
    let _: Option<&dyn StockMovementRepository> = None;
}
