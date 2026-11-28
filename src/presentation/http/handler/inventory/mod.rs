use std::sync::Arc;

use axum::Router;

use crate::presentation::http::Dependencies;

mod stock;
mod stock_errmap;

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    stock::routes()
}
