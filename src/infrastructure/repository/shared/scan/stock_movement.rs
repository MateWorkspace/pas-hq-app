use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::{MovementReason, StockMovement};

pub(crate) fn decode(row: &PgRow) -> Result<StockMovement, sqlx::Error> {
    let reason: String = row.try_get("reason")?;

    Ok(StockMovement {
        id: row.try_get("id")?,
        sku_id: row.try_get("sku_id")?,
        sku_label: row.try_get("sku_label")?,
        src_location_id: row.try_get("src_location_id")?,
        src_location_name: row.try_get("src_location_name")?,
        dest_location_id: row.try_get("dest_location_id")?,
        dest_location_name: row.try_get("dest_location_name")?,
        quantity: row.try_get("quantity")?,
        reason: reason
            .parse::<MovementReason>()
            .map_err(|_| sqlx::Error::Decode("unknown movement reason".into()))?,
        created_at: row.try_get("created_at")?,
    })
}
