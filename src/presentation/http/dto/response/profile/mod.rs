use serde::Serialize;

use crate::domain::models::entity::User;

use super::model::AuditResponse;

#[derive(Clone, Debug, Serialize)]
pub struct AccountResponse {
    pub id: i64,
    pub role_id: i64,
    pub name: String,
    pub bio: String,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub audit: AuditResponse,
}

pub(crate) fn account(value: User) -> AccountResponse {
    AccountResponse {
        id: value.id,
        role_id: value.role_id,
        name: value.name,
        bio: value.bio,
        avatar_path: value.avatar_path,
        is_active: value.is_active,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}
