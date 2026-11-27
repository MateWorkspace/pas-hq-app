use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::Inventory;

pub(crate) fn decode(row: &PgRow) -> Result<Inventory, sqlx::Error> {
    Ok(Inventory {
        id: row.try_get("id")?,
        location_id: row.try_get("location_id")?,
        sku_id: row.try_get("sku_id")?,
        stock: row.try_get("stock")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
