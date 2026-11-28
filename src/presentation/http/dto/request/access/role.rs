use serde::Deserialize;

use crate::domain::usecases::{
    Page,
    access::role::{CreateRole, GrantPermission, ListRoles, UpdateRole},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateRoleRequest {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_default: bool,
}

impl From<CreateRoleRequest> for CreateRole {
    fn from(value: CreateRoleRequest) -> Self {
        Self {
            slug: value.slug,
            name: value.name,
            description: value.description,
            is_default: value.is_default,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
}

impl UpdateRoleRequest {
    pub(crate) fn into_update(self, id: i64) -> UpdateRole {
        UpdateRole {
            id,
            name: self.name,
            description: self.description,
            is_default: self.is_default,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListRolesQuery {
    pub is_default: Option<bool>,
    pub name_contains: Option<String>,
}

impl ListRolesQuery {
    pub(crate) fn into_list(self, page: Page) -> ListRoles {
        ListRoles {
            is_default: self.is_default,
            name_contains: self.name_contains,
            page,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrantPermissionRequest {
    pub permission_id: i64,
}

impl GrantPermissionRequest {
    pub(crate) fn into_grant(self, role_id: i64) -> GrantPermission {
        GrantPermission {
            role_id,
            permission_id: self.permission_id,
        }
    }
}
