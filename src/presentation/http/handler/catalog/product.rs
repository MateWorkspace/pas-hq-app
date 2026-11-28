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
            catalog::product::{CreateProductRequest, ListProductsQuery, UpdateProductRequest},
            path::IdPath,
        },
        response::{catalog::ProductListResponse, error::ApiError, page::PageResponse},
    },
    middleware::RequestContext,
};

use super::product_errmap::{
    create_product_error, delete_product_error, get_product_error, list_products_error,
    update_product_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/products", get(list_products).post(create_product))
        .route(
            "/v1/products/{id}",
            get(get_product)
                .patch(update_product)
                .delete(delete_product),
        )
}

async fn create_product(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<CreateProductRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let product = dependencies
        .manage_product
        .create_product(&context, request.into())
        .await
        .map_err(create_product_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::catalog::product(
            product,
        )),
    ))
}

async fn update_product(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    payload: Result<Json<UpdateProductRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Json(request) = payload?;

    let product = dependencies
        .manage_product
        .update_product(&context, request.into_update(path.id))
        .await
        .map_err(update_product_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::catalog::product(product),
    ))
}

async fn delete_product(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    dependencies
        .manage_product
        .delete_product(&context, path.id)
        .await
        .map_err(delete_product_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_products(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<ListProductsQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(request) = query?;
    let page =
        crate::presentation::http::dto::request::page::page_from(request.limit, request.offset);

    let products = dependencies
        .manage_product
        .list_products(&context, request.into_list())
        .await
        .map_err(list_products_error)?;

    Ok(Json(ProductListResponse {
        data: products
            .into_iter()
            .map(crate::presentation::http::dto::response::catalog::product)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn get_product(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let product = dependencies
        .manage_product
        .get_product(&context, path.id)
        .await
        .map_err(get_product_error)?;

    Ok(Json(
        crate::presentation::http::dto::response::catalog::product(product),
    ))
}
