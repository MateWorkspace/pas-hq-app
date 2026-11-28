use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::auth::session::{RefreshRequest, SignInRequest},
        response::{
            auth::{SessionResponse, WhoamiResponse},
            error::ApiError,
        },
    },
    middleware::RequestContext,
};

use super::session_errmap::{refresh_error, sign_in_error, whoami_error};

/// Credential exchange. Runs under an open context because no identity exists yet.
pub(crate) fn public_routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/auth/sign-in", post(sign_in))
        .route("/v1/auth/refresh", post(refresh))
}

/// Session reads. Runs under the authenticated context so the caller's own
/// identity and permissions can be resolved.
pub(crate) fn protected_routes() -> Router<Arc<Dependencies>> {
    Router::new().route("/v1/auth/me", get(whoami))
}

async fn sign_in(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<SignInRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;
    let tokens = dependencies
        .session
        .sign_in(&context, request.into())
        .await
        .map_err(sign_in_error)?;

    Ok((StatusCode::OK, Json(SessionResponse::from(tokens))))
}

async fn refresh(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<RefreshRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;
    let tokens = dependencies
        .session
        .refresh(&context, request.into())
        .await
        .map_err(refresh_error)?;

    Ok(Json(SessionResponse::from(tokens)))
}

async fn whoami(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
) -> Result<impl IntoResponse, ApiError> {
    let snapshot = dependencies
        .session
        .whoami(&context)
        .await
        .map_err(whoami_error)?;

    Ok(Json(WhoamiResponse::from(snapshot)))
}
