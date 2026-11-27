use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::ProductVariant},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for product variant dimensions.
///
/// A duplicate name within a product is reported as `PRODUCT_VARIANT_NAME_CONFLICT`;
/// a missing variant as `PRODUCT_VARIANT_NOT_FOUND`. Deleting a variant cascades its
/// selections; the application must regenerate the product's SKUs afterwards.
pub trait ProductVariantRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ProductVariant>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ProductVariant>>;

    fn list_by_product<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_id: i64,
    ) -> RepositoryFuture<'a, Vec<ProductVariant>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, ProductVariant>;

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
    pub name: String,
    pub created_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub updated_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListArgs {
    pub pagination: Pagination,
}
