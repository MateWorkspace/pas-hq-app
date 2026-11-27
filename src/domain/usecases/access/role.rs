use crate::domain::models::{AppContext, entity::Role};

use super::super::{Page, UsecaseFuture};

/// Administrative operations for roles and their permission grants.
///
/// Slugs are stable and omitted from updates. At most one role may be the default;
/// making a second one default is rejected.
pub trait ManageRole: Send + Sync {
    fn create_role<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateRole,
    ) -> UsecaseFuture<'a, Role>;

    fn update_role<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateRole,
    ) -> UsecaseFuture<'a, Role>;

    fn delete_role<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;

    fn list_roles<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListRoles,
    ) -> UsecaseFuture<'a, Vec<Role>>;

    fn get_role_permissions<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, Vec<String>>;

    fn grant_permission<'a>(
        &'a self,
        context: &'a AppContext,
        input: GrantPermission,
    ) -> UsecaseFuture<'a, ()>;

    fn revoke_permission<'a>(
        &'a self,
        context: &'a AppContext,
        input: GrantPermission,
    ) -> UsecaseFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateRole {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub is_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateRole {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListRoles {
    pub is_default: Option<bool>,
    pub name_contains: Option<String>,
    pub page: Page,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrantPermission {
    pub role_id: i64,
    pub permission_id: i64,
}
