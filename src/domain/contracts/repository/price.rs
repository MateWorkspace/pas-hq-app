use bigdecimal::BigDecimal;

use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::Price},
};

use super::RepositoryFuture;

/// Persistence for SKU prices.
///
/// A `location_id` of `None` addresses the global price; `Some` addresses an outlet
/// override. A duplicate is reported as `PRICE_CONFLICT`; a missing price as
/// `PRICE_NOT_FOUND`.
pub trait PriceRepository: Send + Sync {
    /// Insert or replace the price for its exact scope (global or one location).
    fn set<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: SetArgs,
    ) -> RepositoryFuture<'a, Price>;

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        location_id: Option<i64>,
        currency: &'a str,
    ) -> RepositoryFuture<'a, Option<Price>>;

    /// Return the outlet override when present, otherwise the global price.
    fn find_effective<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        location_id: i64,
        currency: &'a str,
    ) -> RepositoryFuture<'a, Option<Price>>;

    fn list_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
    ) -> RepositoryFuture<'a, Vec<Price>>;

    fn remove<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        location_id: Option<i64>,
        currency: &'a str,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetArgs {
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
    pub amount: BigDecimal,
    pub updated_by: Option<i64>,
}
