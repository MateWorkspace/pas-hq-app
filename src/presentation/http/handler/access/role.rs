use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch},
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::{
            access::role::{
                CreateRoleRequest, GrantPermissionRequest, ListRolesQuery, UpdateRoleRequest,
            },
            path::IdPath,
        },
        response::{
            access::{PermissionSlugListResponse, RoleListResponse},
            error::ApiError,
            page::PageResponse,
        },
    },
    middleware::RequestContext,
};

use super::role_errmap::{
    create_role_error, delete_role_error, get_role_permissions_error, grant_permission_error,
    list_roles_error, revoke_permission_error, update_role_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/roles", get(list_roles).post(create_role))
        .route("/v1/roles/{id}", patch(update_role).delete(delete_role))
        .route(
            "/v1/roles/{id}/permissions",
            get(get_role_permissions).post(grant_permission),
        )
        .route(
            "/v1/roles/{id}/permissions/{permission_id}",
            delete(revoke_permission),
        )
}

async fn create_role(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateRoleRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let role = dependencies
        .manage_role
        .create_role(&context, request.into())
        .await
        .map_err(create_role_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::access::role(role)),
    ))
}

async fn update_role(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<UpdateRoleRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let role = dependencies
        .manage_role
        .update_role(&context, request.into_update(path.id))
        .await
        .map_err(update_role_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::access::role(role),
    ))
}

async fn delete_role(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_role
        .delete_role(&context, path.id)
        .await
        .map_err(delete_role_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_roles(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<ListRolesQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(request) = query?;
    let page =
        crate::presentation::http::dto::request::page::page_from(request.limit, request.offset);

    let roles = dependencies
        .manage_role
        .list_roles(&context, request.into_list())
        .await
        .map_err(list_roles_error)?;

    Ok(Json(RoleListResponse {
        data: roles
            .into_iter()
            .map(crate::presentation::http::dto::response::access::role)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn get_role_permissions(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let slugs = dependencies
        .manage_role
        .get_role_permissions(&context, path.id)
        .await
        .map_err(get_role_permissions_error)?;

    Ok(Json(PermissionSlugListResponse { data: slugs }))
}

async fn grant_permission(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<GrantPermissionRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    dependencies
        .manage_role
        .grant_permission(&context, request.into_grant(path.id))
        .await
        .map_err(grant_permission_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn revoke_permission(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<(IdPath, PermissionIdPath)>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path((role, permission)) = path?;

    dependencies
        .manage_role
        .revoke_permission(
            &context,
            crate::domain::usecases::access::role::GrantPermission {
                role_id: role.id,
                permission_id: permission.permission_id,
            },
        )
        .await
        .map_err(revoke_permission_error)?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct PermissionIdPath {
    pub permission_id: i64,
}
