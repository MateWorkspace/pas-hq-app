use crate::domain::models::{AppContext, entity::ApiKey};

use super::super::{Page, UsecaseFuture};
use crate::domain::usecases::auth::api_key::IssuedApiKey;

/// An administrator managing API keys across users.
///
/// Every method requires an `apikey:*` permission. The raw key is returned only by
/// `create` and `rotate`; listings never expose it.
pub trait ManageApiKey: Send + Sync {
    fn list_api_keys<'a>(
        &'a self,
        context: &'a AppContext,
        page: Page,
    ) -> UsecaseFuture<'a, Vec<ApiKey>>;

    fn list_user_api_keys<'a>(
        &'a self,
        context: &'a AppContext,
        input: ListUserApiKeys,
    ) -> UsecaseFuture<'a, Vec<ApiKey>>;

    fn create_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateApiKey,
    ) -> UsecaseFuture<'a, IssuedApiKey>;

    fn rotate_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, IssuedApiKey>;

    fn set_api_key_active<'a>(
        &'a self,
        context: &'a AppContext,
        input: SetApiKeyActive,
    ) -> UsecaseFuture<'a, ApiKey>;

    fn delete_api_key<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateApiKey {
    pub user_id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListUserApiKeys {
    pub user_id: i64,
    pub page: Page,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SetApiKeyActive {
    pub id: i64,
    pub is_active: bool,
}
