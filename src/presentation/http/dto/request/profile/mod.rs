use serde::Deserialize;

use crate::domain::usecases::profile::{account::UpdateAccount, password::ChangePasswordInput};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_path: Option<String>,
}

impl From<UpdateAccountRequest> for UpdateAccount {
    fn from(value: UpdateAccountRequest) -> Self {
        Self {
            name: value.name,
            bio: value.bio,
            avatar_path: value.avatar_path,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

impl From<ChangePasswordRequest> for ChangePasswordInput {
    fn from(value: ChangePasswordRequest) -> Self {
        Self {
            current_password: value.current_password,
            new_password: value.new_password,
        }
    }
}
