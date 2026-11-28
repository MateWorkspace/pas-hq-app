use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::{
            access::user::{CreateUserRequest, ListUsersQuery, UpdateUserRequest},
            path::IdPath,
        },
        response::{access::UserListResponse, error::ApiError, page::PageResponse},
    },
    middleware::RequestContext,
};

use super::user_errmap::{
    create_user_error, delete_user_error, get_user_error, list_users_error, update_user_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/users", get(list_users).post(create_user))
        .route(
            "/v1/users/{id}",
            get(get_user).patch(update_user).delete(delete_user),
        )
}

async fn create_user(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateUserRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let user = dependencies
        .manage_user
        .create_user(&context, request.into())
        .await
        .map_err(create_user_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::access::user(user)),
    ))
}

async fn update_user(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<UpdateUserRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let request = request.into_update(path.id).map_err(|message| {
        ApiError::bad_request("SUSPENDED_UNTIL_INVALID", format!("The {message}."))
    })?;

    let user = dependencies
        .manage_user
        .update_user(&context, request)
        .await
        .map_err(update_user_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::access::user(user),
    ))
}

async fn delete_user(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_user
        .delete_user(&context, path.id)
        .await
        .map_err(delete_user_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_users(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<ListUsersQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(request) = query?;
    let page =
        crate::presentation::http::dto::request::page::page_from(request.limit, request.offset);

    let users = dependencies
        .manage_user
        .list_users(&context, request.into_list())
        .await
        .map_err(list_users_error)?;

    Ok(Json(UserListResponse {
        data: users
            .into_iter()
            .map(crate::presentation::http::dto::response::access::user)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn get_user(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let user = dependencies
        .manage_user
        .get_user(&context, path.id)
        .await
        .map_err(get_user_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::access::user(user),
    ))
}
