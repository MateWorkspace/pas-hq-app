use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::Price;

pub(crate) fn decode(row: &PgRow) -> Result<Price, sqlx::Error> {
    Ok(Price {
        id: row.try_get("id")?,
        sku_id: row.try_get("sku_id")?,
        location_id: row.try_get("location_id")?,
        currency: row.try_get("currency")?,
        amount: row.try_get("amount")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
