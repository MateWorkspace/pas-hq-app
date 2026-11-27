use crate::domain::models::{AppContext, entity::Price};

use super::super::UsecaseFuture;

/// Read and maintain SKU prices.
///
/// A request with no location addresses the global price; a location addresses an
/// outlet override. Reads resolve the effective price with the documented fallback
/// from the outlet override to the global row.
pub trait ManagePrice: Send + Sync {
    fn set_price<'a>(
        &'a self,
        context: &'a AppContext,
        input: SetPrice,
    ) -> UsecaseFuture<'a, Price>;

    fn remove_price<'a>(
        &'a self,
        context: &'a AppContext,
        input: PriceKey,
    ) -> UsecaseFuture<'a, ()>;

    /// List every price row for a SKU: global first, then per-location overrides.
    fn list_prices<'a>(
        &'a self,
        context: &'a AppContext,
        sku_id: i64,
    ) -> UsecaseFuture<'a, Vec<Price>>;

    /// Resolve the price a location would charge, falling back to the global row.
    fn get_effective_price<'a>(
        &'a self,
        context: &'a AppContext,
        input: EffectivePriceRequest,
    ) -> UsecaseFuture<'a, Option<Price>>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPrice {
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
    pub amount: bigdecimal::BigDecimal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriceKey {
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectivePriceRequest {
    pub sku_id: i64,
    pub location_id: i64,
    pub currency: String,
}
