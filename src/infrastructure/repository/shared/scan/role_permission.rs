use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::RolePermission;

pub(crate) fn decode(row: &PgRow) -> Result<RolePermission, sqlx::Error> {
    Ok(RolePermission {
        id: row.try_get("id")?,
        role_id: row.try_get("role_id")?,
        permission_id: row.try_get("permission_id")?,
        created_at: row.try_get("created_at")?,
        created_by: row.try_get("created_by")?,
    })
}
