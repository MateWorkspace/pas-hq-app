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
            api_key::{CreateApiKeyRequest, SetActiveRequest},
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
    create_error, delete_error, list_error, list_user_error, rotate_error, set_active_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/api-keys", get(list_api_keys).post(create_api_key))
        .route(
            "/v1/api-keys/{id}",
            patch(set_api_key_active).delete(delete_api_key),
        )
        .route("/v1/api-keys/{id}/rotate", post(rotate_api_key))
        .route("/v1/users/{id}/api-keys", get(list_user_api_keys))
}

async fn list_api_keys(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(page) = query?;
    let page = page.into_page();

    let keys = dependencies
        .manage_api_key
        .list_api_keys(&context, page)
        .await
        .map_err(list_error)?;

    Ok(Json(ApiKeyListResponse {
        data: keys
            .into_iter()
            .map(crate::presentation::http::dto::response::api_key::api_key)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn list_user_api_keys(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Query(page) = query?;
    let page = page.into_page();

    let keys = dependencies
        .manage_api_key
        .list_user_api_keys(
            &context,
            crate::domain::usecases::access::api_key::ListUserApiKeys {
                user_id: path.id,
                page,
            },
        )
        .await
        .map_err(list_user_error)?;

    Ok(Json(ApiKeyListResponse {
        data: keys
            .into_iter()
            .map(crate::presentation::http::dto::response::api_key::api_key)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn create_api_key(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateApiKeyRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let issued = dependencies
        .manage_api_key
        .create_api_key(&context, request.into())
        .await
        .map_err(create_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::api_key::issued(
            issued,
        )),
    ))
}

async fn rotate_api_key(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let issued = dependencies
        .manage_api_key
        .rotate_api_key(&context, path.id)
        .await
        .map_err(rotate_error)?;

    Ok(Json(IssuedApiKeyResponse {
        key: issued.raw,
        api_key: crate::presentation::http::dto::response::api_key::api_key(issued.key),
    }))
}

async fn set_api_key_active(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<SetActiveRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let key = dependencies
        .manage_api_key
        .set_api_key_active(
            &context,
            crate::domain::usecases::access::api_key::SetApiKeyActive {
                id: path.id,
                is_active: request.is_active,
            },
        )
        .await
        .map_err(set_active_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::api_key::api_key(key),
    ))
}

async fn delete_api_key(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_api_key
        .delete_api_key(&context, path.id)
        .await
        .map_err(delete_error)?;

    Ok(StatusCode::NO_CONTENT)
}
