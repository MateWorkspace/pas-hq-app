use serde::Serialize;

use crate::domain::models::entity::{
    Price, Product, ProductSku, ProductVariant, ProductVariantSelection,
};
use crate::domain::usecases::catalog::sku::SkuRegenerationPlan;

use super::{model::AuditResponse, page::PageResponse};

#[derive(Clone, Debug, Serialize)]
pub struct ProductResponse {
    pub id: i64,
    pub brand: String,
    pub model: String,
    pub manufacturer: String,
    pub description: String,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductListResponse {
    pub data: Vec<ProductResponse>,
    pub page: PageResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct VariantResponse {
    pub id: i64,
    pub product_id: i64,
    pub name: String,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct SelectionResponse {
    pub id: i64,
    pub product_variant_id: i64,
    pub name: String,
    pub description: String,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct SkuResponse {
    pub id: i64,
    pub product_id: i64,
    pub selection_key: String,
    pub code: Option<String>,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct PriceResponse {
    pub id: i64,
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
    /// Exact decimal rendered as text.
    pub amount: String,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegenerationPlanResponse {
    pub created_keys: Vec<String>,
    pub removed_skus: Vec<RemovedSkuResponse>,
    pub retained_keys: Vec<String>,
    pub destroys_stock: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct RemovedSkuResponse {
    pub sku_id: i64,
    pub selection_key: String,
    pub stock_on_hand: i64,
}

pub(crate) fn product(value: Product) -> ProductResponse {
    ProductResponse {
        id: value.id,
        brand: value.brand,
        model: value.model,
        manufacturer: value.manufacturer,
        description: value.description,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn variant(value: ProductVariant) -> VariantResponse {
    VariantResponse {
        id: value.id,
        product_id: value.product_id,
        name: value.name,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn selection(value: ProductVariantSelection) -> SelectionResponse {
    SelectionResponse {
        id: value.id,
        product_variant_id: value.product_variant_id,
        name: value.name,
        description: value.description,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn sku(value: ProductSku) -> SkuResponse {
    SkuResponse {
        id: value.id,
        product_id: value.product_id,
        selection_key: value.selection_key,
        code: value.code,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn price(value: Price) -> PriceResponse {
    PriceResponse {
        id: value.id,
        sku_id: value.sku_id,
        location_id: value.location_id,
        currency: value.currency,
        amount: value.amount.to_string(),
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn regeneration_plan(value: SkuRegenerationPlan) -> RegenerationPlanResponse {
    RegenerationPlanResponse {
        created_keys: value.created_keys,
        removed_skus: value
            .removed_skus
            .into_iter()
            .map(|sku| RemovedSkuResponse {
                sku_id: sku.sku_id,
                selection_key: sku.selection_key,
                stock_on_hand: sku.stock_on_hand,
            })
            .collect(),
        retained_keys: value.retained_keys,
        destroys_stock: value.destroys_stock,
    }
}
