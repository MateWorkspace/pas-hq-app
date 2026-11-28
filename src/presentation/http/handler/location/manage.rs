use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::{
            inventory::location::{
                CreateLocationRequest, ListLocationsQuery, UpdateLocationRequest,
            },
            page::PageQuery,
            path::IdPath,
        },
        response::{error::ApiError, location::LocationListResponse, page::PageResponse},
    },
    middleware::RequestContext,
};

use super::location_errmap::{
    create_location_error, deactivate_location_error, delete_location_error, get_location_error,
    list_locations_error, update_location_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/locations", get(list_locations).post(create_location))
        .route(
            "/v1/locations/{id}",
            get(get_location)
                .patch(update_location)
                .delete(delete_location),
        )
        .route("/v1/locations/{id}/deactivate", post(deactivate_location))
}

async fn create_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateLocationRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let request = request.into_create().map_err(|message| {
        ApiError::bad_request("LOCATION_KIND_INVALID", format!("The {message}."))
    })?;

    let location = dependencies
        .manage_location
        .create_location(&context, request)
        .await
        .map_err(create_location_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::inventory::location(location)),
    ))
}

async fn update_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<UpdateLocationRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let request = request.into_update(path.id).map_err(|message| {
        ApiError::bad_request("LOCATION_KIND_INVALID", format!("The {message}."))
    })?;

    let location = dependencies
        .manage_location
        .update_location(&context, request)
        .await
        .map_err(update_location_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::inventory::location(location),
    ))
}

async fn deactivate_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let location = dependencies
        .manage_location
        .deactivate_location(&context, path.id)
        .await
        .map_err(deactivate_location_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::inventory::location(location),
    ))
}

async fn delete_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_location
        .delete_location(&context, path.id)
        .await
        .map_err(delete_location_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_locations(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<(ListLocationsQuery, PageQuery)>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query((request, page)) = query?;
    let page = page.into_page();

    let request = request.into_list(page).map_err(|message| {
        ApiError::bad_request("LOCATION_KIND_INVALID", format!("The {message}."))
    })?;

    let locations = dependencies
        .manage_location
        .list_locations(&context, request)
        .await
        .map_err(list_locations_error)?;

    Ok(Json(LocationListResponse {
        data: locations
            .into_iter()
            .map(crate::presentation::http::dto::response::inventory::location)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn get_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let location = dependencies
        .manage_location
        .get_location(&context, path.id)
        .await
        .map_err(get_location_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::inventory::location(location),
    ))
}
