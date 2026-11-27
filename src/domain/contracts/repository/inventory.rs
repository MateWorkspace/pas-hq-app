use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::Inventory},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for the current stock of a SKU at a location.
///
/// A duplicate location/SKU pair is reported as `INVENTORY_CONFLICT`; a missing row
/// as `INVENTORY_NOT_FOUND`. Stock is the running balance; changes should also be
/// recorded through `StockMovementRepository`.
pub trait InventoryRepository: Send + Sync {
    /// Insert or replace the stock for an exact location/SKU pair.
    fn set_stock<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: SetStockArgs,
    ) -> RepositoryFuture<'a, Inventory>;

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        location_id: i64,
        sku_id: i64,
    ) -> RepositoryFuture<'a, Option<Inventory>>;

    fn list_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        location_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<Inventory>>;

    fn list_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<Inventory>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetStockArgs {
    pub location_id: i64,
    pub sku_id: i64,
    pub stock: i64,
    pub updated_by: Option<i64>,
}
