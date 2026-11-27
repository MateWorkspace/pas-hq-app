use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::Product;

pub(crate) fn decode(row: &PgRow) -> Result<Product, sqlx::Error> {
    Ok(Product {
        id: row.try_get("id")?,
        brand: row.try_get("brand")?,
        model: row.try_get("model")?,
        manufacturer: row.try_get("manufacturer")?,
        description: row.try_get("description")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
