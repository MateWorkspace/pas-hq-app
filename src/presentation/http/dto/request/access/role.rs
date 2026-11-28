use serde::Deserialize;

use crate::domain::usecases::access::role::{CreateRole, GrantPermission, ListRoles, UpdateRole};

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

/// List filter with paging.
///
/// Paging is declared here instead of flattened from `PageQuery`: the query
/// extractor uses `serde_urlencoded`, which cannot deserialize `flatten`.
#[derive(Clone, Debug, Deserialize)]
pub struct ListRolesQuery {
    pub is_default: Option<bool>,
    pub name_contains: Option<String>,
    #[serde(default = "crate::presentation::http::dto::request::page::default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

impl ListRolesQuery {
    pub(crate) fn into_list(self) -> ListRoles {
        ListRoles {
            is_default: self.is_default,
            name_contains: self.name_contains,
            page: crate::presentation::http::dto::request::page::page_from(self.limit, self.offset),
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
