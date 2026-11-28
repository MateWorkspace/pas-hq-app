use std::sync::Arc;

use axum::Router;

use crate::presentation::http::Dependencies;

pub(crate) mod api_key;
pub(crate) mod api_key_errmap;
pub(crate) mod permission;
pub(crate) mod permission_errmap;
pub(crate) mod role;
pub(crate) mod role_errmap;
pub(crate) mod user;
pub(crate) mod user_errmap;

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .merge(role::routes())
        .merge(permission::routes())
        .merge(user::routes())
        .merge(api_key::routes())
}
