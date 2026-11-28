use serde::Serialize;

use crate::domain::models::entity::{Role, User};

use super::{model::AuditResponse, page::PageResponse};

#[derive(Clone, Debug, Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub is_default: bool,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct RoleListResponse {
    pub data: Vec<RoleResponse>,
    pub page: PageResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct PermissionSlugListResponse {
    pub data: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PermissionListResponse {
    pub data: Vec<PermissionResponse>,
    pub page: PageResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub role_id: i64,
    pub name: String,
    pub bio: String,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub suspended_until: Option<String>,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserListResponse {
    pub data: Vec<UserResponse>,
    pub page: PageResponse,
}

pub(crate) fn role(value: Role) -> RoleResponse {
    RoleResponse {
        id: value.id,
        slug: value.slug,
        name: value.name,
        description: value.description,
        is_default: value.is_default,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn permission(value: crate::domain::models::entity::Permission) -> PermissionResponse {
    PermissionResponse {
        id: value.id,
        slug: value.slug,
        name: value.name,
        description: value.description,
        created_at: value.created_at.to_rfc3339(),
    }
}

pub(crate) fn user(value: User) -> UserResponse {
    UserResponse {
        id: value.id,
        role_id: value.role_id,
        name: value.name,
        bio: value.bio,
        avatar_path: value.avatar_path,
        is_active: value.is_active,
        suspended_until: value.suspended_until.map(|at| at.to_rfc3339()),
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}
