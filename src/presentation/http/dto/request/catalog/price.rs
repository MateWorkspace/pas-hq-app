use serde::Deserialize;

use crate::domain::usecases::catalog::price::{EffectivePriceRequest, PriceKey, SetPrice};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetPriceRequest {
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
    /// Exact decimal as text so no precision is lost in transit.
    pub amount: String,
}

impl SetPriceRequest {
    pub(crate) fn into_set(self) -> Result<SetPrice, String> {
        let amount = self
            .amount
            .parse::<bigdecimal::BigDecimal>()
            .map_err(|_| "amount must be a decimal number".to_owned())?;

        Ok(SetPrice {
            sku_id: self.sku_id,
            location_id: self.location_id,
            currency: self.currency,
            amount,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemovePriceQuery {
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
}

impl RemovePriceQuery {
    pub(crate) fn into_key(self) -> PriceKey {
        PriceKey {
            sku_id: self.sku_id,
            location_id: self.location_id,
            currency: self.currency,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectivePriceQuery {
    pub sku_id: i64,
    pub location_id: i64,
    pub currency: String,
}

impl EffectivePriceQuery {
    pub(crate) fn into_request(self) -> EffectivePriceRequest {
        EffectivePriceRequest {
            sku_id: self.sku_id,
            location_id: self.location_id,
            currency: self.currency,
        }
    }
}
