use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::Product},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for product models.
///
/// A duplicate brand/model/manufacturer is reported as
/// `PRODUCT_BRAND_MODEL_MANUFACTURER_CONFLICT`; a missing product as
/// `PRODUCT_NOT_FOUND`. Deleting a product cascades its variants, SKUs, prices,
/// inventories, and movement references.
pub trait ProductRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, Product>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<Product>>;

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<Product>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, Product>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub brand: String,
    pub model: String,
    pub manufacturer: String,
    pub description: String,
    pub created_by: Option<i64>,
}

/// Fields left `None` are unchanged.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgs {
    pub brand: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
    pub updated_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListArgs {
    pub brand_contains: Option<String>,
    pub name_contains: Option<String>,
    pub pagination: Pagination,
}
