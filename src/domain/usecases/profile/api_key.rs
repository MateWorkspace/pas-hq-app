use crate::domain::models::{AppContext, entity::ApiKey};

use super::super::{Page, UsecaseFuture};
use crate::domain::usecases::auth::api_key::IssuedApiKey;

/// A user managing their own API keys.
///
/// The target is always the signed-in user, so no permission guard applies. The raw
/// key is returned only by `create` and `rotate`.
pub trait ManageOwnApiKey: Send + Sync {
    fn list_own_api_keys<'a>(
        &'a self,
        context: &'a AppContext,
        page: Page,
    ) -> UsecaseFuture<'a, Vec<ApiKey>>;

    fn create_own_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateOwnApiKey,
    ) -> UsecaseFuture<'a, IssuedApiKey>;

    fn rotate_own_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, IssuedApiKey>;

    fn set_own_api_key_active<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
        is_active: bool,
    ) -> UsecaseFuture<'a, ApiKey>;

    fn delete_own_api_key<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateOwnApiKey {
    pub name: String,
    pub description: String,
}
