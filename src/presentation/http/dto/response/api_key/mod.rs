use serde::Serialize;

use crate::domain::models::entity::ApiKey;
use crate::domain::usecases::auth::api_key::IssuedApiKey;

use super::{model::AuditResponse, page::PageResponse};

#[derive(Clone, Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub redacted: String,
    pub is_active: bool,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiKeyListResponse {
    pub data: Vec<ApiKeyResponse>,
    pub page: PageResponse,
}

/// A newly issued key. `key` is present in this response only.
#[derive(Clone, Debug, Serialize)]
pub struct IssuedApiKeyResponse {
    pub key: String,
    pub api_key: ApiKeyResponse,
}

pub(crate) fn api_key(value: ApiKey) -> ApiKeyResponse {
    ApiKeyResponse {
        id: value.id,
        user_id: value.user_id,
        name: value.name,
        description: value.description,
        redacted: value.redacted,
        is_active: value.is_active,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn issued(value: IssuedApiKey) -> IssuedApiKeyResponse {
    IssuedApiKeyResponse {
        key: value.raw,
        api_key: api_key(value.key),
    }
}
