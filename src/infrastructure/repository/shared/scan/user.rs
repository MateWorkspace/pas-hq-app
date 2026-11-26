use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::User;

pub(crate) fn decode(row: &PgRow) -> Result<User, sqlx::Error> {
    Ok(User {
        id: row.try_get("id")?,
        role_id: row.try_get("role_id")?,
        name: row.try_get("name")?,
        bio: row.try_get("bio")?,
        avatar_path: row.try_get("avatar_path")?,
        is_active: row.try_get("is_active")?,
        suspended_until: row.try_get("suspended_until")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
