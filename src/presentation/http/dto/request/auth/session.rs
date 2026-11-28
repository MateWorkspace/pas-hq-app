use serde::Deserialize;

use crate::domain::usecases::auth::{RefreshInput, SignInInput};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

impl From<SignInRequest> for SignInInput {
    fn from(value: SignInRequest) -> Self {
        Self {
            username: value.username,
            password: value.password,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

impl From<RefreshRequest> for RefreshInput {
    fn from(value: RefreshRequest) -> Self {
        Self {
            refresh_token: value.refresh_token,
        }
    }
}
