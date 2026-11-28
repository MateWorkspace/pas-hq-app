use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
};

use crate::presentation::http::{
    Dependencies,
    dto::{
        request::{
            catalog::variant::{
                CreateSelectionRequest, CreateVariantRequest, UpdateSelectionRequest,
                UpdateVariantRequest,
            },
            path::IdPath,
        },
        response::error::ApiError,
    },
    middleware::RequestContext,
};

use super::variant_errmap::{
    create_selection_error, create_variant_error, delete_selection_error, delete_variant_error,
    list_selections_error, list_variants_error, update_selection_error, update_variant_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/variants", post(create_variant))
        .route(
            "/v1/variants/{id}",
            patch(update_variant).delete(delete_variant),
        )
        .route("/v1/products/{id}/variants", get(list_variants))
        .route("/v1/selections", post(create_selection))
        .route(
            "/v1/selections/{id}",
            patch(update_selection).delete(delete_selection),
        )
        .route("/v1/variants/{id}/selections", get(list_selections))
}

async fn create_variant(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateVariantRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let variant = dependencies
        .manage_variant
        .create_variant(&context, request.into())
        .await
        .map_err(create_variant_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::catalog::variant(
            variant,
        )),
    ))
}

async fn update_variant(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<UpdateVariantRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let variant = dependencies
        .manage_variant
        .update_variant(&context, request.into_update(path.id))
        .await
        .map_err(update_variant_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::catalog::variant(variant),
    ))
}

async fn delete_variant(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_variant
        .delete_variant(&context, path.id)
        .await
        .map_err(delete_variant_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_variants(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let variants = dependencies
        .manage_variant
        .list_variants(&context, path.id)
        .await
        .map_err(list_variants_error)?;

    Ok(Json(
        variants
            .into_iter()
            .map(crate::presentation::http::dto::response::catalog::variant)
            .collect::<Vec<_>>(),
    ))
}

async fn create_selection(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateSelectionRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let selection = dependencies
        .manage_variant_selection
        .create_selection(&context, request.into())
        .await
        .map_err(create_selection_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::catalog::selection(selection)),
    ))
}

async fn update_selection(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<UpdateSelectionRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let selection = dependencies
        .manage_variant_selection
        .update_selection(&context, request.into_update(path.id))
        .await
        .map_err(update_selection_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::catalog::selection(selection),
    ))
}

async fn delete_selection(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_variant_selection
        .delete_selection(&context, path.id)
        .await
        .map_err(delete_selection_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_selections(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let selections = dependencies
        .manage_variant_selection
        .list_selections(&context, path.id)
        .await
        .map_err(list_selections_error)?;

    Ok(Json(
        selections
            .into_iter()
            .map(crate::presentation::http::dto::response::catalog::selection)
            .collect::<Vec<_>>(),
    ))
}
