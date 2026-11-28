use serde::Deserialize;

use crate::domain::usecases::catalog::variant::{
    CreateSelection, CreateVariant, UpdateSelection, UpdateVariant,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateVariantRequest {
    pub product_id: i64,
    pub name: String,
}

impl From<CreateVariantRequest> for CreateVariant {
    fn from(value: CreateVariantRequest) -> Self {
        Self {
            product_id: value.product_id,
            name: value.name,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateVariantRequest {
    pub name: Option<String>,
}

impl UpdateVariantRequest {
    pub(crate) fn into_update(self, id: i64) -> UpdateVariant {
        UpdateVariant {
            id,
            name: self.name,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSelectionRequest {
    pub product_variant_id: i64,
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl From<CreateSelectionRequest> for CreateSelection {
    fn from(value: CreateSelectionRequest) -> Self {
        Self {
            product_variant_id: value.product_variant_id,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateSelectionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateSelectionRequest {
    pub(crate) fn into_update(self, id: i64) -> UpdateSelection {
        UpdateSelection {
            id,
            name: self.name,
            description: self.description,
        }
    }
}
