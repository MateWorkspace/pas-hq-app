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
        response::{auth::ClaimsResponse, error::ApiError},
    },
    middleware::RequestContext,
};

use super::auth_errmap::{refresh_error, sign_in_error};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/auth/sign-in", post(sign_in))
        .route("/v1/auth/refresh", post(refresh))
        .route("/v1/auth/me", get(current_user))
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

    Ok((
        StatusCode::OK,
        Json(
            crate::presentation::http::dto::response::auth::SessionResponse {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                user: crate::presentation::http::dto::response::auth::SessionUserResponse {
                    id: tokens.claims.user_id,
                    name: tokens.claims.name,
                },
            },
        ),
    ))
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

    Ok(Json(
        crate::presentation::http::dto::response::auth::SessionResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            user: crate::presentation::http::dto::response::auth::SessionUserResponse {
                id: tokens.claims.user_id,
                name: tokens.claims.name,
            },
        },
    ))
}

async fn current_user(
    RequestContext(context): RequestContext,
) -> Result<impl IntoResponse, ApiError> {
    let Some(claims) = context.claims.as_ref() else {
        return Err(ApiError::unauthorized(
            "AUTHENTICATION_REQUIRED",
            "You must be signed in to view your session.",
        ));
    };

    Ok(Json(ClaimsResponse::from(claims)))
}
