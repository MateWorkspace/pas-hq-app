use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::BaseAuth;

pub(crate) fn decode(row: &PgRow) -> Result<BaseAuth, sqlx::Error> {
    Ok(BaseAuth {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        username: row.try_get("username")?,
        password_hash: row.try_get("password_hash")?,
        email: row.try_get("email")?,
        phone: row.try_get("phone")?,
        is_email_verified: row.try_get("is_email_verified")?,
        is_phone_verified: row.try_get("is_phone_verified")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
