use chrono::{DateTime, Utc};

use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AccessClaims, AppContext, entity::ApiKey},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for user API keys.
///
/// Only the digest and preview are stored. A duplicate name within a user is
/// reported as `API_KEY_NAME_CONFLICT`; a missing key as `API_KEY_NOT_FOUND`.
pub trait ApiKeyRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ApiKey>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ApiKey>>;

    fn list_by_user<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        user_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<ApiKey>>;

    /// Every key in the installation, for administrator views.
    fn list_all<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<ApiKey>>;

    /// Replace the digest and preview of an existing key, keeping its identity.
    fn rotate<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        input: RotateArgs,
    ) -> RepositoryFuture<'a, ApiKey>;

    fn set_active<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        is_active: bool,
        updated_by: Option<i64>,
    ) -> RepositoryFuture<'a, ApiKey>;

    /// Delete one key, returning it so the caller can invalidate its cache entry.
    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ApiKey>;

    /// Delete every key of one user, returning the removed digests.
    fn delete_by_user_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        user_id: i64,
    ) -> RepositoryFuture<'a, Vec<String>>;

    /// Resolve the identity behind a key digest in one read.
    fn find_auth_by_hash<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        hashed_key: &'a str,
    ) -> RepositoryFuture<'a, Option<ApiKeyAuth>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub redacted: String,
    pub hashed_key: String,
    pub created_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RotateArgs {
    pub redacted: String,
    pub hashed_key: String,
    pub updated_by: Option<i64>,
}

/// Everything a key validation needs, resolved in one read.
///
/// Kept flat so the auth path fetches only what the claims require and can apply
/// the same liveness rules as a password sign-in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiKeyAuth {
    pub api_key_id: i64,
    pub key_is_active: bool,
    pub user_id: i64,
    pub user_name: String,
    pub user_is_active: bool,
    pub suspended_until: Option<DateTime<Utc>>,
    pub role_id: i64,
    pub role_name: String,
    pub role_slug: String,
}

impl ApiKeyAuth {
    /// True when both the key and its owner may authenticate right now.
    pub fn is_usable(&self, now: DateTime<Utc>) -> bool {
        self.key_is_active
            && self.user_is_active
            && self.suspended_until.is_none_or(|until| until <= now)
    }

    pub fn claims(&self) -> AccessClaims {
        AccessClaims {
            user_id: self.user_id,
            name: self.user_name.clone(),
            role_id: self.role_id,
            role_name: self.role_name.clone(),
            role: self.role_slug.clone(),
        }
    }
}
