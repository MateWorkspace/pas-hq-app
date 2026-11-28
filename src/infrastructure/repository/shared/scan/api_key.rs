use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::ApiKey;

pub(crate) fn decode(row: &PgRow) -> Result<ApiKey, sqlx::Error> {
    Ok(ApiKey {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        redacted: row.try_get("redacted")?,
        hashed_key: row.try_get("hashed_key")?,
        is_active: row.try_get("is_active")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
