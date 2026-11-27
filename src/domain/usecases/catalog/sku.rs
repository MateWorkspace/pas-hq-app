use crate::domain::models::{AppContext, entity::ProductSku};

use super::super::UsecaseFuture;

/// Read and manual maintenance access to concrete SKUs.
///
/// SKUs are normally generated, not authored. `RegenerateProductSkus` owns the
/// diff; this trait exposes the results and a guarded manual delete.
pub trait ManageSku: Send + Sync {
    fn list_skus<'a>(
        &'a self,
        context: &'a AppContext,
        product_id: i64,
    ) -> UsecaseFuture<'a, Vec<ProductSku>>;

    /// Set or clear the optional business code for one SKU.
    fn set_sku_code<'a>(
        &'a self,
        context: &'a AppContext,
        input: SetSkuCode,
    ) -> UsecaseFuture<'a, ProductSku>;

    /// Delete one SKU. A SKU that still holds stock must be handled first.
    fn delete_sku<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetSkuCode {
    pub sku_id: i64,
    /// `None` clears the code.
    pub code: Option<String>,
}

/// Reconcile a product's SKUs with its current variant design.
///
/// The operation computes the valid selection-key set and inserts missing SKUs,
/// leaves existing ones untouched (so their prices and stock survive), and deletes
/// the ones that disappeared. `dry_run` returns the plan without writing, so callers
/// can show the affected SKUs and stock on hand before committing.
pub trait RegenerateProductSkus: Send + Sync {
    fn regenerate_product_skus<'a>(
        &'a self,
        context: &'a AppContext,
        input: RegenerateProductSkusInput,
    ) -> UsecaseFuture<'a, SkuRegenerationPlan>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegenerateProductSkusInput {
    pub product_id: i64,
    pub dry_run: bool,
}

/// What a regeneration would do, or did.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkuRegenerationPlan {
    /// Selection keys that would be created.
    pub created_keys: Vec<String>,
    /// Selection keys that would be removed, with the SKU identity retained.
    pub removed_skus: Vec<RemovedSku>,
    /// Selection keys left untouched.
    pub retained_keys: Vec<String>,
    /// True when stock would be destroyed by the removals.
    pub destroys_stock: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemovedSku {
    pub sku_id: i64,
    pub selection_key: String,
    /// Total units currently held across all locations.
    pub stock_on_hand: i64,
}
