use std::fmt;

use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::BaseAuth},
};

use super::RepositoryFuture;

/// Persistence for username/password credentials.
///
/// Each user has at most one credential record. A duplicate username, email,
/// phone, or user link is reported as `BASE_AUTH_USERNAME_CONFLICT`,
/// `BASE_AUTH_EMAIL_CONFLICT`, `BASE_AUTH_PHONE_CONFLICT`, or
/// `BASE_AUTH_USER_CONFLICT`; a missing record is `BASE_AUTH_NOT_FOUND`.
/// Implementations never log the password hash.
pub trait BaseAuthRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, BaseAuth>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<BaseAuth>>;

    fn find_by_user_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        user_id: i64,
    ) -> RepositoryFuture<'a, Option<BaseAuth>>;

    fn find_by_username<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        username: &'a str,
    ) -> RepositoryFuture<'a, Option<BaseAuth>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, BaseAuth>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Eq, PartialEq)]
pub struct CreateArgs {
    pub user_id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_by: Option<i64>,
}

/// Fields left `None` are unchanged.
#[derive(Clone, Eq, PartialEq)]
pub struct UpdateArgs {
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_email_verified: Option<bool>,
    pub is_phone_verified: Option<bool>,
    pub updated_by: Option<i64>,
}

impl fmt::Debug for CreateArgs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CreateArgs")
            .field("user_id", &self.user_id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("phone", &self.phone)
            .field("created_by", &self.created_by)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for UpdateArgs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UpdateArgs")
            .field("username", &self.username)
            .field("email", &self.email)
            .field("phone", &self.phone)
            .field("is_email_verified", &self.is_email_verified)
            .field("is_phone_verified", &self.is_phone_verified)
            .field("updated_by", &self.updated_by)
            .finish_non_exhaustive()
    }
}
