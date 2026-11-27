use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::ProductSku;

pub(crate) fn decode(row: &PgRow) -> Result<ProductSku, sqlx::Error> {
    Ok(ProductSku {
        id: row.try_get("id")?,
        product_id: row.try_get("product_id")?,
        selection_key: row.try_get("selection_key")?,
        code: row.try_get("code")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
