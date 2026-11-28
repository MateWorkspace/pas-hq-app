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
            catalog::sku::{RegenerateQuery, SetSkuCodeRequest},
            path::IdPath,
        },
        response::error::ApiError,
    },
    middleware::RequestContext,
};

use super::sku_errmap::{
    delete_sku_error, list_skus_error, regenerate_skus_error, set_sku_code_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/products/{id}/skus", get(list_skus))
        .route("/v1/products/{id}/skus/regenerate", post(regenerate_skus))
        .route("/v1/skus/{id}", patch(set_sku_code).delete(delete_sku))
}

async fn list_skus(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let skus = dependencies
        .manage_sku
        .list_skus(&context, path.id)
        .await
        .map_err(list_skus_error)?;

    Ok(Json(
        skus.into_iter()
            .map(crate::presentation::http::dto::response::catalog::sku)
            .collect::<Vec<_>>(),
    ))
}

async fn set_sku_code(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<SetSkuCodeRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let sku = dependencies
        .manage_sku
        .set_sku_code(&context, request.into_set(path.id))
        .await
        .map_err(set_sku_code_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::catalog::sku(sku),
    ))
}

async fn delete_sku(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_sku
        .delete_sku(&context, path.id)
        .await
        .map_err(delete_sku_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn regenerate_skus(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    query: Result<Query<RegenerateQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Query(query) = query?;

    let plan = dependencies
        .regenerate_product_skus
        .regenerate_product_skus(&context, query.into_input(path.id))
        .await
        .map_err(regenerate_skus_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::catalog::regeneration_plan(plan),
    ))
}
