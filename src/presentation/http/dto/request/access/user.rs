use serde::Deserialize;

use crate::domain::usecases::access::{
    permission::{CreatePermission, ListPermissions},
    user::{CreateUser, ListUsers, UpdateUser},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatePermissionRequest {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl From<CreatePermissionRequest> for CreatePermission {
    fn from(value: CreatePermissionRequest) -> Self {
        Self {
            slug: value.slug,
            name: value.name,
            description: value.description,
        }
    }
}

/// List filter with paging.
///
/// Paging is declared here instead of flattened from `PageQuery`: the query
/// extractor uses `serde_urlencoded`, which cannot deserialize `flatten`.
#[derive(Clone, Debug, Deserialize)]
pub struct ListPermissionsQuery {
    pub name_contains: Option<String>,
    #[serde(default = "crate::presentation::http::dto::request::page::default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

impl ListPermissionsQuery {
    pub(crate) fn into_list(self) -> ListPermissions {
        ListPermissions {
            name_contains: self.name_contains,
            page: crate::presentation::http::dto::request::page::page_from(self.limit, self.offset),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUserRequest {
    pub role_id: i64,
    pub name: String,
    #[serde(default)]
    pub bio: String,
    pub avatar_path: Option<String>,
    #[serde(default = "default_active")]
    pub is_active: bool,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl From<CreateUserRequest> for CreateUser {
    fn from(value: CreateUserRequest) -> Self {
        Self {
            role_id: value.role_id,
            name: value.name,
            bio: value.bio,
            avatar_path: value.avatar_path,
            is_active: value.is_active,
            username: value.username,
            password: value.password,
            email: value.email,
            phone: value.phone,
        }
    }
}

const fn default_active() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRequest {
    pub role_id: Option<i64>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_path: Option<String>,
    pub is_active: Option<bool>,
    /// RFC 3339 timestamp, or `null`.
    pub suspended_until: Option<String>,
    #[serde(default)]
    pub clear_suspended_until: bool,
}

impl UpdateUserRequest {
    pub(crate) fn into_update(self, id: i64) -> Result<UpdateUser, String> {
        let suspended_until = match self.suspended_until {
            Some(value) => Some(
                chrono::DateTime::parse_from_rfc3339(&value)
                    .map_err(|_| "suspended_until must be an RFC 3339 timestamp".to_owned())?
                    .with_timezone(&chrono::Utc),
            ),
            None => None,
        };

        Ok(UpdateUser {
            id,
            role_id: self.role_id,
            name: self.name,
            bio: self.bio,
            avatar_path: self.avatar_path,
            is_active: self.is_active,
            suspended_until,
            clear_suspended_until: self.clear_suspended_until,
        })
    }
}

/// List filter with paging.
///
/// Paging is declared here instead of flattened from `PageQuery`: the query
/// extractor uses `serde_urlencoded`, which cannot deserialize `flatten`.
#[derive(Clone, Debug, Deserialize)]
pub struct ListUsersQuery {
    pub role_id: Option<i64>,
    pub is_active: Option<bool>,
    pub name_contains: Option<String>,
    #[serde(default = "crate::presentation::http::dto::request::page::default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

impl ListUsersQuery {
    pub(crate) fn into_list(self) -> ListUsers {
        ListUsers {
            role_id: self.role_id,
            is_active: self.is_active,
            name_contains: self.name_contains,
            page: crate::presentation::http::dto::request::page::page_from(self.limit, self.offset),
        }
    }
}
