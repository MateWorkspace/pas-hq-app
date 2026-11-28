use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Query, State,
        rejection::{JsonRejection, QueryRejection},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::{access::user::ListPermissionsQuery, page::PageQuery},
        response::{access::PermissionListResponse, error::ApiError, page::PageResponse},
    },
    middleware::RequestContext,
};

use super::permission_errmap::{create_permission_error, list_permissions_error};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new().route(
        "/v1/permissions",
        get(list_permissions).post(create_permission),
    )
}

async fn create_permission(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<
        Json<crate::presentation::http::dto::request::access::user::CreatePermissionRequest>,
        JsonRejection,
    >,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let permission = dependencies
        .manage_permission
        .create_permission(&context, request.into())
        .await
        .map_err(create_permission_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::access::permission(permission)),
    ))
}

async fn list_permissions(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<(ListPermissionsQuery, PageQuery)>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query((request, page)) = query?;
    let page = page.into_page();

    let permissions = dependencies
        .manage_permission
        .list_permissions(&context, request.into_list(page))
        .await
        .map_err(list_permissions_error)?;

    Ok(Json(PermissionListResponse {
        data: permissions
            .into_iter()
            .map(crate::presentation::http::dto::response::access::permission)
            .collect(),
        page: PageResponse::from(page),
    }))
}
