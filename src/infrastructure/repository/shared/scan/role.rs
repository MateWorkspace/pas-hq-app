use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::Role;

pub(crate) fn decode(row: &PgRow) -> Result<Role, sqlx::Error> {
    Ok(Role {
        id: row.try_get("id")?,
        slug: row.try_get("slug")?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        is_default: row.try_get("is_default")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
