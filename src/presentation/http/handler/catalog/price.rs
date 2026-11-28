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
            catalog::price::{EffectivePriceQuery, RemovePriceQuery, SetPriceRequest},
            path::IdPath,
        },
        response::error::ApiError,
    },
    middleware::RequestContext,
};

use super::price_errmap::{
    effective_price_error, list_prices_error, remove_price_error, set_price_error,
};

pub(crate) fn routes() -> Router<Arc<Dependencies>> {
    Router::new()
        .route("/v1/skus/{id}/prices", get(list_prices))
        .route("/v1/prices", post(set_price).delete(remove_price))
        .route("/v1/prices/effective", get(effective_price))
}

async fn set_price(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    payload: Result<Json<SetPriceRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(request) = payload?;

    let request = request.into_set().map_err(|message| {
        ApiError::bad_request("PRICE_AMOUNT_INVALID", format!("The {message}."))
    })?;

    let price = dependencies
        .manage_price
        .set_price(&context, request)
        .await
        .map_err(set_price_error)?;

    Ok((
        StatusCode::CREATED,
        Json(crate::presentation::http::dto::response::catalog::price(
            price,
        )),
    ))
}

async fn remove_price(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<RemovePriceQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query?;

    dependencies
        .manage_price
        .remove_price(&context, query.into_key())
        .await
        .map_err(remove_price_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_prices(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    path: Result<Path<IdPath>, PathRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Path(path) = path?;

    let prices = dependencies
        .manage_price
        .list_prices(&context, path.id)
        .await
        .map_err(list_prices_error)?;

    Ok(Json(
        prices
            .into_iter()
            .map(crate::presentation::http::dto::response::catalog::price)
            .collect::<Vec<_>>(),
    ))
}

async fn effective_price(
    State(dependencies): State<Arc<Dependencies>>,
    RequestContext(context): RequestContext,
    query: Result<Query<EffectivePriceQuery>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query?;

    let price = dependencies
        .manage_price
        .get_effective_price(&context, query.into_request())
        .await
        .map_err(effective_price_error)?;

    match price {
        Some(price) => Ok(
            Json(crate::presentation::http::dto::response::catalog::price(
                price,
            ))
            .into_response(),
        ),
        None => Err(ApiError::not_found(
            "PRICE_NOT_SET",
            "No price is set for this SKU in this currency, at this location or globally.",
        )),
    }
}
