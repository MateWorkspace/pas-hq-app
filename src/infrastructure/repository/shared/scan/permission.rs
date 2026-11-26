use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::Permission;

pub(crate) fn decode(row: &PgRow) -> Result<Permission, sqlx::Error> {
    Ok(Permission {
        id: row.try_get("id")?,
        slug: row.try_get("slug")?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        created_at: row.try_get("created_at")?,
    })
}
