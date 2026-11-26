use pas_hq_app::domain::contracts::repository::{
    BaseAuthRepository, PermissionRepository, RolePermissionRepository, RoleRepository,
    UserRepository,
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
}
