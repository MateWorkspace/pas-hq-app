use serde::Serialize;

use crate::domain::usecases::auth::session::{SessionProfile, SessionTokens, SessionUser};

/// The session body returned by sign-in and refresh.
///
/// Tokens are issued once; the profile travels with them so a client needs no
/// follow-up request.
#[derive(Clone, Debug, Serialize)]
pub struct SessionResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: SessionUserResponse,
    pub permissions: Vec<String>,
}

/// The session body returned by the profile endpoint.
#[derive(Clone, Debug, Serialize)]
pub struct ProfileResponse {
    pub user: SessionUserResponse,
    pub permissions: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SessionUserResponse {
    pub id: i64,
    pub name: String,
    pub role_id: i64,
    pub role_name: String,
    pub role: String,
}

impl From<SessionUser> for SessionUserResponse {
    fn from(value: SessionUser) -> Self {
        Self {
            id: value.id,
            name: value.name,
            role_id: value.role_id,
            role_name: value.role_name,
            role: value.role,
        }
    }
}

impl From<SessionProfile> for ProfileResponse {
    fn from(value: SessionProfile) -> Self {
        Self {
            user: value.user.into(),
            permissions: value.permissions,
        }
    }
}

impl From<SessionTokens> for SessionResponse {
    fn from(value: SessionTokens) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            user: value.profile.user.into(),
            permissions: value.profile.permissions,
        }
    }
}
