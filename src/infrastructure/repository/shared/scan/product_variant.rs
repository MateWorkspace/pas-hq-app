use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::ProductVariant;

pub(crate) fn decode(row: &PgRow) -> Result<ProductVariant, sqlx::Error> {
    Ok(ProductVariant {
        id: row.try_get("id")?,
        product_id: row.try_get("product_id")?,
        name: row.try_get("name")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
