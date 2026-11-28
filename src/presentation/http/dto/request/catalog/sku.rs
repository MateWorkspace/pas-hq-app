use serde::Deserialize;

use crate::domain::usecases::catalog::sku::{RegenerateProductSkusInput, SetSkuCode};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetSkuCodeRequest {
    /// Omit or send `null` to clear the code.
    pub code: Option<String>,
}

impl SetSkuCodeRequest {
    pub(crate) fn into_set(self, sku_id: i64) -> SetSkuCode {
        SetSkuCode {
            sku_id,
            code: self.code,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegenerateQuery {
    #[serde(default)]
    pub dry_run: bool,
}

impl RegenerateQuery {
    pub(crate) fn into_input(self, product_id: i64) -> RegenerateProductSkusInput {
        RegenerateProductSkusInput {
            product_id,
            dry_run: self.dry_run,
        }
    }
}
