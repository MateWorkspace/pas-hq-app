use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::{
            api_key::{CreateOwnApiKeyRequest, SetActiveRequest},
            page::PageQuery,
            path::IdPath,
        },
        response::{
            api_key::{ApiKeyListResponse, IssuedApiKeyResponse},
            error::ApiError,
            page::PageResponse,
        },
    },
    middleware::RequestContext,
};

use super::api_key_errmap::{
    own_create_error, own_delete_error, own_list_error, own_rotate_error, own_set_active_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route(
            "/v1/profile/api-keys",
            get(list_own_api_keys).post(create_own_api_key),
        )
        .route(
            "/v1/profile/api-keys/{id}",
            patch(set_own_api_key_active).delete(delete_own_api_key),
        )
        .route("/v1/profile/api-keys/{id}/rotate", post(rotate_own_api_key))
}

async fn list_own_api_keys(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(page) = query?;
    let page = page.into_page();

    let keys = dependencies
        .manage_own_api_key
        .list_own_api_keys(&context, page)
        .await
        .map_err(own_list_error)?;

    Ok(Json(ApiKeyListResponse {
        data: keys
            .into_iter()
            .map(crate::presentation::http::dto::response::api_key::api_key)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn create_own_api_key(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateOwnApiKeyRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let issued = dependencies
        .manage_own_api_key
        .create_own_api_key(&context, request.into())
        .await
        .map_err(own_create_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::api_key::issued(
            issued,
        )),
    ))
}

async fn rotate_own_api_key(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let issued = dependencies
        .manage_own_api_key
        .rotate_own_api_key(&context, path.id)
        .await
        .map_err(own_rotate_error)?;

    Ok(Json(IssuedApiKeyResponse {
        key: issued.raw,
        api_key: crate::presentation::http::dto::response::api_key::api_key(issued.key),
    }))
}

async fn set_own_api_key_active(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<SetActiveRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let key = dependencies
        .manage_own_api_key
        .set_own_api_key_active(&context, path.id, request.is_active)
        .await
        .map_err(own_set_active_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::api_key::api_key(key),
    ))
}

async fn delete_own_api_key(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_own_api_key
        .delete_own_api_key(&context, path.id)
        .await
        .map_err(own_delete_error)?;

    Ok(StatusCode::NO_CONTENT)
}
