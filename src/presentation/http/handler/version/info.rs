use std::sync::Arc;

use axum::{Json, Router, routing::get};

use crate::{
    domain::models::AppInfo,
    presentation::http::{Dependencies, dto::response::version::VersionResponse},
};

/// Public build metadata. This route has no request body and cannot fail, so it
/// needs no `*_errmap.rs` companion.
pub(crate) fn routes(app_info: AppInfo) -> Router<Arc<Dependencies>> {
    Router::new().route(
        "/v1/version",
        get(move || async move { Json(VersionResponse::from(app_info)) }),
    )
}
