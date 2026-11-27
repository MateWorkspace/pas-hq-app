use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::ProductVariantSelection;

pub(crate) fn decode(row: &PgRow) -> Result<ProductVariantSelection, sqlx::Error> {
    Ok(ProductVariantSelection {
        id: row.try_get("id")?,
        product_variant_id: row.try_get("product_variant_id")?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
