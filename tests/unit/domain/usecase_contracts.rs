use pas_hq_app::domain::usecases::{
    access::{ManagePermission, ManageRole, ManageUser},
    auth::Session,
    catalog::{
        ManagePrice, ManageProduct, ManageSku, ManageVariant, ManageVariantSelection,
        RegenerateProductSkus,
    },
    inventory::{AdjustStock, GetStock, ListMovements, ListStock, RecordMovement},
    location::ManageLocation,
    misc::CreateAppContext,
    profile::{ChangePassword, ManageAccount},
    shared::PermissionGuard,
};

/// Usecase traits must stay usable through trait objects because presentation
/// depends on them rather than concrete implementations. This fails to compile if a
/// contract stops being object-safe.
#[test]
fn usecase_contracts_are_object_safe() {
    let _: Option<&dyn Session> = None;
    let _: Option<&dyn CreateAppContext> = None;
    let _: Option<&dyn PermissionGuard> = None;

    let _: Option<&dyn ManageRole> = None;
    let _: Option<&dyn ManagePermission> = None;
    let _: Option<&dyn ManageUser> = None;

    let _: Option<&dyn ManageAccount> = None;
    let _: Option<&dyn ChangePassword> = None;

    let _: Option<&dyn ManageProduct> = None;
    let _: Option<&dyn ManageVariant> = None;
    let _: Option<&dyn ManageVariantSelection> = None;
    let _: Option<&dyn ManageSku> = None;
    let _: Option<&dyn RegenerateProductSkus> = None;
    let _: Option<&dyn ManagePrice> = None;

    let _: Option<&dyn GetStock> = None;
    let _: Option<&dyn AdjustStock> = None;
    let _: Option<&dyn ListStock> = None;
    let _: Option<&dyn ListMovements> = None;
    let _: Option<&dyn RecordMovement> = None;

    let _: Option<&dyn ManageLocation> = None;
}
