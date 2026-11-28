use axum::Router;
use axum::response::IntoResponse;
use tower_http::catch_panic::CatchPanicLayer;

use crate::presentation::http::dto::response::error::ApiError;

pub(crate) mod api_key;
pub(crate) mod bearer;
pub(crate) mod context;
pub(crate) mod header;
pub(crate) mod trace_id;

pub use context::RequestContext;

/// Turns a panicking handler into a safe 500 instead of dropping the connection.
pub(crate) fn recovery<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(CatchPanicLayer::custom(|_| {
        ApiError::internal(
            "UNEXPECTED_FAILURE",
            "Something went wrong while handling the request. Please try again.",
        )
        .into_response()
    }))
}
