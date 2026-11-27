use crate::domain::{
    contracts::utility::DbExecutor,
    models::{
        AppContext,
        entity::{MovementReason, StockMovement},
    },
};

use super::{Pagination, RepositoryFuture};

/// Persistence for the append-only stock ledger.
///
/// Rows are inserted and read, never updated or deleted. A missing row is reported
/// as `STOCK_MOVEMENT_NOT_FOUND`.
pub trait StockMovementRepository: Send + Sync {
    fn record<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: RecordArgs,
    ) -> RepositoryFuture<'a, StockMovement>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<StockMovement>>;

    fn list_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<StockMovement>>;

    fn list_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        location_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<StockMovement>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordArgs {
    pub sku_id: Option<i64>,
    pub sku_label: Option<String>,
    pub src_location_id: Option<i64>,
    pub src_location_name: Option<String>,
    pub dest_location_id: Option<i64>,
    pub dest_location_name: Option<String>,
    pub quantity: i64,
    pub reason: MovementReason,
}
