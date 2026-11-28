use serde::Deserialize;

use crate::domain::usecases::{
    Page,
    catalog::product::{CreateProduct, ListProducts, UpdateProduct},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateProductRequest {
    pub brand: String,
    pub model: String,
    pub manufacturer: String,
    #[serde(default)]
    pub description: String,
}

impl From<CreateProductRequest> for CreateProduct {
    fn from(value: CreateProductRequest) -> Self {
        Self {
            brand: value.brand,
            model: value.model,
            manufacturer: value.manufacturer,
            description: value.description,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateProductRequest {
    pub brand: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
}

impl UpdateProductRequest {
    pub(crate) fn into_update(self, id: i64) -> UpdateProduct {
        UpdateProduct {
            id,
            brand: self.brand,
            model: self.model,
            manufacturer: self.manufacturer,
            description: self.description,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListProductsQuery {
    pub brand_contains: Option<String>,
    pub name_contains: Option<String>,
}

impl ListProductsQuery {
    pub(crate) fn into_list(self, page: Page) -> ListProducts {
        ListProducts {
            brand_contains: self.brand_contains,
            name_contains: self.name_contains,
            page,
        }
    }
}
