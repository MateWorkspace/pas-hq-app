use std::sync::Arc;

use axum::Router;

use crate::presentation::http::Dependencies;

pub(crate) mod price;
pub(crate) mod price_errmap;
pub(crate) mod product;
pub(crate) mod product_errmap;
pub(crate) mod sku;
pub(crate) mod sku_errmap;
pub(crate) mod variant;
pub(crate) mod variant_errmap;

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .merge(product::routes())
        .merge(variant::routes())
        .merge(sku::routes())
        .merge(price::routes())
}
