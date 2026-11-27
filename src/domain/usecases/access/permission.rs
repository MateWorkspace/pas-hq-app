use crate::domain::models::{AppContext, entity::Permission};

use super::super::{Page, UsecaseFuture};

/// Administrative operations for authorization actions.
///
/// Permissions are created and read only; they are never updated or removed, so
/// their slugs stay stable for grant references.
pub trait ManagePermission: Send + Sync {
    fn create_permission<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreatePermission,
    ) -> UsecaseFuture<'a, Permission>;

    fn list_permissions<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListPermissions,
    ) -> UsecaseFuture<'a, Vec<Permission>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatePermission {
    pub slug: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListPermissions {
    pub name_contains: Option<String>,
    pub page: Page,
}
