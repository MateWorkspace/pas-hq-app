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
            inventory::stock::{AdjustStockRequest, TransferStockRequest},
            page::PageQuery,
        },
        response::{
            error::ApiError,
            inventory::{InventoryListResponse, InventoryResponse, MovementListResponse},
            page::PageResponse,
        },
    },
    middleware::RequestContext,
};

use super::stock_errmap::{
    adjust_stock_error, list_movements_error, list_stock_error, transfer_stock_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/stock/adjustments", post(adjust_stock))
        .route("/v1/stock/transfers", post(transfer_stock))
        .route("/v1/stock/by-location/{id}", get(list_by_location))
        .route("/v1/stock/by-sku/{id}", get(list_by_sku))
        .route(
            "/v1/stock/movements/by-sku/{id}",
            get(list_movements_by_sku),
        )
        .route(
            "/v1/stock/movements/by-location/{id}",
            get(list_movements_by_location),
        )
}

async fn adjust_stock(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<AdjustStockRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let request = request.into_input().map_err(|message| {
        ApiError::bad_request("STOCK_REASON_INVALID", format!("The {message}."))
    })?;

    let inventory = dependencies
        .adjust_stock
        .adjust_stock(&context, request)
        .await
        .map_err(adjust_stock_error)?;

    Ok(Json(InventoryResponse {
        id: inventory.id,
        location_id: inventory.location_id,
        sku_id: inventory.sku_id,
        stock: inventory.stock,
        audit: crate::presentation::http::dto::response::model::AuditResponse::new(
            inventory.created_at,
            inventory.updated_at,
        ),
    }))
}

async fn transfer_stock(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<TransferStockRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let movement = dependencies
        .record_movement
        .transfer_stock(&context, request.into())
        .await
        .map_err(transfer_stock_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::inventory::movement(movement)),
    ))
}

async fn list_by_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Query(page) = query?;
    let page = page.into_page();

    let items = dependencies
        .get_stock
        .list_stock_by_location(
            &context,
            crate::domain::usecases::inventory::stock::StockByLocation {
                location_id: path.id,
                page,
            },
        )
        .await
        .map_err(list_stock_error)?;

    Ok(Json(InventoryListResponse {
        data: items
            .into_iter()
            .map(crate::presentation::http::dto::response::inventory::inventory)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn list_by_sku(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Query(page) = query?;
    let page = page.into_page();

    let items = dependencies
        .get_stock
        .list_stock_by_sku(
            &context,
            crate::domain::usecases::inventory::stock::StockBySku {
                sku_id: path.id,
                page,
            },
        )
        .await
        .map_err(list_stock_error)?;

    Ok(Json(InventoryListResponse {
        data: items
            .into_iter()
            .map(crate::presentation::http::dto::response::inventory::inventory)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn list_movements_by_sku(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Query(page) = query?;
    let page = page.into_page();

    let items = dependencies
        .list_stock
        .list_stock_movements_by_sku(
            &context,
            crate::domain::usecases::inventory::stock::MovementBySku {
                sku_id: path.id,
                page,
            },
        )
        .await
        .map_err(list_movements_error)?;

    Ok(Json(MovementListResponse {
        data: items
            .into_iter()
            .map(crate::presentation::http::dto::response::inventory::movement)
            .collect(),
        page: PageResponse::from(page),
    }))
}

async fn list_movements_by_location(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;
    let Query(page) = query?;
    let page = page.into_page();

    let items = dependencies
        .list_stock
        .list_stock_movements_by_location(
            &context,
            crate::domain::usecases::inventory::stock::MovementByLocation {
                location_id: path.id,
                page,
            },
        )
        .await
        .map_err(list_movements_error)?;

    Ok(Json(MovementListResponse {
        data: items
            .into_iter()
            .map(crate::presentation::http::dto::response::inventory::movement)
            .collect(),
        page: PageResponse::from(page),
    }))
}

pub(crate) use crate::presentation::http::dto::request::path::IdPath;
