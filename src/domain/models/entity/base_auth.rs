use std::fmt;

use chrono::{DateTime, Utc};

/// Username/password credentials for exactly one user.
///
/// Passwords are persisted only as one-way hashes; the hash is never exposed by
/// `Debug` or logged.
#[derive(Clone, Eq, PartialEq)]
pub struct BaseAuth {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}

impl fmt::Debug for BaseAuth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BaseAuth")
            .field("id", &self.id)
            .field("user_id", &self.user_id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("phone", &self.phone)
            .field("is_email_verified", &self.is_email_verified)
            .field("is_phone_verified", &self.is_phone_verified)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("created_by", &self.created_by)
            .field("updated_by", &self.updated_by)
            .finish_non_exhaustive()
    }
}
