use crate::domain::models::{AppContext, entity::User};

use super::super::{Page, UsecaseFuture};

/// Administrative operations for user identities.
///
/// Creating a user also creates its credential record in the same operation, so a
/// user is never left without authentication data. Assigning a role is explicit and
/// the role must exist.
pub trait ManageUser: Send + Sync {
    fn create_user<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateUser,
    ) -> UsecaseFuture<'a, User>;

    fn update_user<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateUser,
    ) -> UsecaseFuture<'a, User>;

    fn delete_user<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;

    fn list_users<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListUsers,
    ) -> UsecaseFuture<'a, Vec<User>>;

    fn get_user<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, User>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateUser {
    pub role_id: i64,
    pub name: String,
    pub bio: String,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    /// Credentials created with the user.
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// Fields left `None` are unchanged; `clear_suspended_until` lifts a suspension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateUser {
    pub id: i64,
    pub role_id: Option<i64>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_path: Option<String>,
    pub is_active: Option<bool>,
    pub suspended_until: Option<chrono::DateTime<chrono::Utc>>,
    pub clear_suspended_until: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListUsers {
    pub role_id: Option<i64>,
    pub is_active: Option<bool>,
    pub name_contains: Option<String>,
    pub page: Page,
}
