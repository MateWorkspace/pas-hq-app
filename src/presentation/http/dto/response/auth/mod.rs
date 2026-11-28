use serde::Serialize;

use crate::domain::models::AccessClaims;

#[derive(Clone, Debug, Serialize)]
pub struct SessionResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: SessionUserResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct SessionUserResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClaimsResponse {
    pub user_id: i64,
    pub name: String,
    pub role_id: i64,
    pub role_name: String,
    pub role: String,
}

impl From<&AccessClaims> for ClaimsResponse {
    fn from(value: &AccessClaims) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name.clone(),
            role_id: value.role_id,
            role_name: value.role_name.clone(),
            role: value.role.clone(),
        }
    }
}
