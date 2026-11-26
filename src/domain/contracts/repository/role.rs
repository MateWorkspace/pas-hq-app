use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::Role},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for role identities.
///
/// Slugs are unique and reserved once created, and at most one role may be the
/// default. A duplicate slug is reported as `ROLE_SLUG_CONFLICT`; a missing role
/// is reported as `ROLE_NOT_FOUND`. When `executor` is absent the operation runs
/// through the pool the repository owns.
pub trait RoleRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, Role>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<Role>>;

    fn find_by_slug<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        slug: &'a str,
    ) -> RepositoryFuture<'a, Option<Role>>;

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<Role>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, Role>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub is_default: bool,
    pub created_by: Option<i64>,
}

/// Fields left `None` are unchanged.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
    pub updated_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListArgs {
    pub is_default: Option<bool>,
    pub name_contains: Option<String>,
    pub pagination: Pagination,
}
