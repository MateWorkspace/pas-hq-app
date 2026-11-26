use chrono::{DateTime, Utc};

use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::User},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for application identities.
///
/// Every user references exactly one role; a missing role is reported as
/// `USER_ROLE_NOT_FOUND`. A missing user is reported as `USER_NOT_FOUND`.
pub trait UserRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, User>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<User>>;

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<User>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, User>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub role_id: i64,
    pub name: String,
    pub bio: String,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub created_by: Option<i64>,
}

/// Fields left `None` are unchanged. `clear_suspended_until` takes precedence over
/// `suspended_until`, setting the suspension to NULL; there is no way to clear
/// `avatar_path` yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgs {
    pub role_id: Option<i64>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_path: Option<String>,
    pub is_active: Option<bool>,
    pub suspended_until: Option<DateTime<Utc>>,
    pub clear_suspended_until: bool,
    pub updated_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListArgs {
    pub role_id: Option<i64>,
    pub is_active: Option<bool>,
    pub name_contains: Option<String>,
    pub pagination: Pagination,
}
