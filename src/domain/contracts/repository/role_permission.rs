use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::RolePermission},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for role-to-permission grants.
///
/// A role/permission pair is unique; granting an existing pair is reported as
/// `ROLE_PERMISSION_CONFLICT` and revoking a missing grant as
/// `ROLE_PERMISSION_NOT_FOUND`.
pub trait RolePermissionRepository: Send + Sync {
    fn grant<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: GrantArgs,
    ) -> RepositoryFuture<'a, RolePermission>;

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        permission_id: i64,
    ) -> RepositoryFuture<'a, Option<RolePermission>>;

    fn list_by_role<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<RolePermission>>;

    fn revoke<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        permission_id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrantArgs {
    pub role_id: i64,
    pub permission_id: i64,
    pub created_by: Option<i64>,
}
