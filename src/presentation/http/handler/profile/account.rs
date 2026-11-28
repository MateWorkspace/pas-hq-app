use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::profile::{ChangePasswordRequest, UpdateAccountRequest},
        response::error::ApiError,
    },
    middleware::RequestContext,
};

use super::profile_errmap::{change_password_error, get_account_error, update_account_error};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/profile", get(get_account).patch(update_account))
        .route("/v1/profile/password", put(change_password))
}

async fn get_account(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
) -> Result<impl IntoResponse, ApiError> {
    let user = dependencies
        .manage_account
        .get_account(&context)
        .await
        .map_err(get_account_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::profile::account(user),
    ))
}

async fn update_account(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<UpdateAccountRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let user = dependencies
        .manage_account
        .update_account(&context, request.into())
        .await
        .map_err(update_account_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::profile::account(user),
    ))
}

async fn change_password(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<ChangePasswordRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    dependencies
        .change_password
        .change_password(&context, request.into())
        .await
        .map_err(change_password_error)?;

    Ok(StatusCode::NO_CONTENT)
}
