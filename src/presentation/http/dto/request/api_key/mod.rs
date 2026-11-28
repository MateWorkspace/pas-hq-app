use serde::Deserialize;

use crate::domain::usecases::{access::api_key::CreateApiKey, profile::api_key::CreateOwnApiKey};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOwnApiKeyRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl From<CreateOwnApiKeyRequest> for CreateOwnApiKey {
    fn from(value: CreateOwnApiKeyRequest) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateApiKeyRequest {
    pub user_id: i64,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl From<CreateApiKeyRequest> for CreateApiKey {
    fn from(value: CreateApiKeyRequest) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetActiveRequest {
    pub is_active: bool,
}
