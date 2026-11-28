use serde::Deserialize;

use crate::domain::{models::MovementReason, usecases::inventory::stock::AdjustStockInput};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdjustStockRequest {
    pub location_id: i64,
    pub sku_id: i64,
    /// Signed delta: positive adds stock, negative removes it.
    pub quantity: i64,
    pub reason: String,
    #[serde(default)]
    pub note: String,
}

impl AdjustStockRequest {
    pub(crate) fn into_input(self) -> Result<AdjustStockInput, String> {
        let reason = self.reason.parse::<MovementReason>().map_err(|_| {
            "reason must be receipt, sale, transfer, adjustment, write_off, or return".to_owned()
        })?;

        Ok(AdjustStockInput {
            location_id: self.location_id,
            sku_id: self.sku_id,
            quantity: self.quantity,
            reason,
            note: self.note,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransferStockRequest {
    pub sku_id: i64,
    pub src_location_id: i64,
    pub dest_location_id: i64,
    pub quantity: i64,
    #[serde(default)]
    pub note: String,
}

impl From<TransferStockRequest> for crate::domain::usecases::inventory::movement::TransferStock {
    fn from(value: TransferStockRequest) -> Self {
        Self {
            sku_id: value.sku_id,
            src_location_id: value.src_location_id,
            dest_location_id: value.dest_location_id,
            quantity: value.quantity,
            note: value.note,
        }
    }
}
