use crate::domain::models::{AppContext, entity::Product};

use super::super::{Page, UsecaseFuture};

/// Product CRUD for the catalog.
///
/// Brand, model, and manufacturer together identify a product and may be updated.
/// Deleting a product removes its variants, SKUs, prices, and stock through
/// cascades; the implementation must guard stock and warn before deleting.
pub trait ManageProduct: Send + Sync {
    fn create_product<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateProduct,
    ) -> UsecaseFuture<'a, Product>;

    fn update_product<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateProduct,
    ) -> UsecaseFuture<'a, Product>;

    fn delete_product<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;

    fn list_products<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListProducts,
    ) -> UsecaseFuture<'a, Vec<Product>>;

    fn get_product<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, Product>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateProduct {
    pub brand: String,
    pub model: String,
    pub manufacturer: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateProduct {
    pub id: i64,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListProducts {
    pub brand_contains: Option<String>,
    pub name_contains: Option<String>,
    pub page: Page,
}
