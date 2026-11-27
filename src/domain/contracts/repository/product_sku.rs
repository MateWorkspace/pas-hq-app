use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::ProductSku},
};

use super::RepositoryFuture;

/// Persistence for concrete sellable units.
///
/// A SKU is created with its variant selections in one operation, so it is never
/// incomplete. A duplicate `selection_key` or `code` is reported as
/// `PRODUCT_SKU_SELECTION_CONFLICT` or `PRODUCT_SKU_CODE_CONFLICT`; a missing SKU as
/// `PRODUCT_SKU_NOT_FOUND`. Deleting a SKU cascades its prices, inventories, and
/// stock-movement references.
pub trait ProductSkuRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ProductSku>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ProductSku>>;

    fn find_by_selection_key<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_id: i64,
        selection_key: &'a str,
    ) -> RepositoryFuture<'a, Option<ProductSku>>;

    fn list_by_product<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_id: i64,
    ) -> RepositoryFuture<'a, Vec<ProductSku>>;

    fn list_selection_ids<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
    ) -> RepositoryFuture<'a, Vec<i64>>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub product_id: i64,
    pub selection_key: String,
    pub code: Option<String>,
    pub variant_selection_ids: Vec<i64>,
    pub created_by: Option<i64>,
}
