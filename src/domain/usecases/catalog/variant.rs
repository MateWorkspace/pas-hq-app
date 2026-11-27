use crate::domain::models::{
    AppContext,
    entity::{ProductVariant, ProductVariantSelection},
};

use super::super::UsecaseFuture;

/// Variant dimensions of a product, such as Color or Storage.
///
/// Any change invalidates the product's SKUs, so the implementation must regenerate
/// them afterwards (see `RegenerateProductSkus`).
pub trait ManageVariant: Send + Sync {
    fn create_variant<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateVariant,
    ) -> UsecaseFuture<'a, ProductVariant>;

    fn update_variant<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateVariant,
    ) -> UsecaseFuture<'a, ProductVariant>;

    fn delete_variant<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;

    fn list_variants<'a>(
        &'a self,
        context: &'a AppContext,
        product_id: i64,
    ) -> UsecaseFuture<'a, Vec<ProductVariant>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateVariant {
    pub product_id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateVariant {
    pub id: i64,
    pub name: Option<String>,
}

/// Selectable values of a variant dimension, such as Black or 256GB.
pub trait ManageVariantSelection: Send + Sync {
    fn create_selection<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateSelection,
    ) -> UsecaseFuture<'a, ProductVariantSelection>;

    fn update_selection<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateSelection,
    ) -> UsecaseFuture<'a, ProductVariantSelection>;

    fn delete_selection<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;

    fn list_selections<'a>(
        &'a self,
        context: &'a AppContext,
        product_variant_id: i64,
    ) -> UsecaseFuture<'a, Vec<ProductVariantSelection>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateSelection {
    pub product_variant_id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateSelection {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}
