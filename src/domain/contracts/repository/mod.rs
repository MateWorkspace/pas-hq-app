pub mod base_auth;
pub mod future;
pub mod permission;
pub mod role;
pub mod role_permission;
pub mod user;

pub use base_auth::BaseAuthRepository;
pub use future::RepositoryFuture;
pub use permission::PermissionRepository;
pub use role::RoleRepository;
pub use role_permission::RolePermissionRepository;
pub use user::UserRepository;

/// Offset pagination shared by list operations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}
