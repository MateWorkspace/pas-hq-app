use crate::domain::models::{AppContext, entity::StockMovement};

use super::super::UsecaseFuture;

/// Transfer stock between two locations.
///
/// Both legs — leaving the source and arriving at the destination — are written in
/// one transaction together with their ledger entries, so stock can never be in
/// transit without a record. The source must hold enough stock, and the two
/// locations must differ.
pub trait RecordMovement: Send + Sync {
    fn transfer_stock<'a>(
        &'a self,
        context: &'a AppContext,
        input: TransferStock,
    ) -> UsecaseFuture<'a, StockMovement>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferStock {
    pub sku_id: i64,
    pub src_location_id: i64,
    pub dest_location_id: i64,
    pub quantity: i64,
    pub note: String,
}

/// Read operations over the movement ledger are grouped with stock reads
/// (`inventory::ListStock`).
pub trait ListMovements: Send + Sync {
    fn list_movements<'a>(
        &'a self,
        context: &'a AppContext,
        request: super::stock::MovementBySku,
    ) -> UsecaseFuture<'a, Vec<StockMovement>>;
}
