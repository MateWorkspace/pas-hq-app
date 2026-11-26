use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::Permission},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for authorization actions.
///
/// Permissions are created only; the slug is unique and never reused. A duplicate
/// slug is reported as `PERMISSION_SLUG_CONFLICT`; a missing permission is
/// reported as `PERMISSION_NOT_FOUND`.
pub trait PermissionRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, Permission>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<Permission>>;

    fn find_by_slug<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        slug: &'a str,
    ) -> RepositoryFuture<'a, Option<Permission>>;

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<Permission>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub slug: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListArgs {
    pub name_contains: Option<String>,
    pub pagination: Pagination,
}
