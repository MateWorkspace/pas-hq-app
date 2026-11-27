use crate::domain::models::{AppContext, entity::Inventory};

use super::super::{Page, UsecaseFuture};

/// Read current stock levels.
pub trait GetStock: Send + Sync {
    fn get_stock<'a>(
        &'a self,
        context: &'a AppContext,
        input: StockKey,
    ) -> UsecaseFuture<'a, Option<Inventory>>;

    fn list_stock_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        request: StockByLocation,
    ) -> UsecaseFuture<'a, Vec<Inventory>>;

    fn list_stock_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        request: StockBySku,
    ) -> UsecaseFuture<'a, Vec<Inventory>>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StockKey {
    pub location_id: i64,
    pub sku_id: i64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StockByLocation {
    pub location_id: i64,
    pub page: Page,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StockBySku {
    pub sku_id: i64,
    pub page: Page,
}

/// Apply a signed change to stock and record the movement beside it.
///
/// The adjustment and its ledger entry are written in one transaction. A result that
/// would drive stock negative is rejected rather than clamped.
pub trait AdjustStock: Send + Sync {
    fn adjust_stock<'a>(
        &'a self,
        context: &'a AppContext,
        input: AdjustStockInput,
    ) -> UsecaseFuture<'a, Inventory>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjustStockInput {
    pub location_id: i64,
    pub sku_id: i64,
    /// Signed delta: positive adds stock, negative removes it.
    pub quantity: i64,
    pub reason: crate::domain::models::MovementReason,
    pub note: String,
}

/// Read the stock ledger.
pub trait ListStock: Send + Sync {
    fn list_stock_movements_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        request: MovementBySku,
    ) -> UsecaseFuture<'a, Vec<crate::domain::models::StockMovement>>;

    fn list_stock_movements_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        request: MovementByLocation,
    ) -> UsecaseFuture<'a, Vec<crate::domain::models::StockMovement>>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MovementBySku {
    pub sku_id: i64,
    pub page: Page,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MovementByLocation {
    pub location_id: i64,
    pub page: Page,
}
