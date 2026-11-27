use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::ProductVariantSelection},
};

use super::RepositoryFuture;

/// Persistence for the selectable values of a variant dimension.
///
/// A duplicate name within a variant is reported as
/// `PRODUCT_VARIANT_SELECTION_NAME_CONFLICT`; a missing selection as
/// `PRODUCT_VARIANT_SELECTION_NOT_FOUND`. Deleting a selection cascades its SKU
/// links; the application must regenerate the product's SKUs afterwards.
pub trait ProductVariantSelectionRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ProductVariantSelection>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ProductVariantSelection>>;

    fn list_by_variant<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_variant_id: i64,
    ) -> RepositoryFuture<'a, Vec<ProductVariantSelection>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, ProductVariantSelection>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgs {
    pub product_variant_id: i64,
    pub name: String,
    pub description: String,
    pub created_by: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub description: Option<String>,
    pub updated_by: Option<i64>,
}
