mod account;
mod api_key;
mod api_key_errmap;
mod profile_errmap;

use std::sync::Arc;

use axum::Router;

use crate::presentation::http::Dependencies;

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .merge(account::routes())
        .merge(api_key::routes())
}
