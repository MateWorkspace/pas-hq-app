use std::fmt;

use chrono::{DateTime, Utc};

/// A long-lived credential for one user.
///
/// Only the digest is stored, so `Debug` omits `hashed_key` the same way `BaseAuth`
/// omits its password hash. The raw key is never part of this value.
#[derive(Clone, Eq, PartialEq)]
pub struct ApiKey {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub redacted: String,
    pub hashed_key: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApiKey")
            .field("id", &self.id)
            .field("user_id", &self.user_id)
            .field("name", &self.name)
            .field("description", &self.description)
            .field("redacted", &self.redacted)
            .field("is_active", &self.is_active)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("created_by", &self.created_by)
            .field("updated_by", &self.updated_by)
            .finish_non_exhaustive()
    }
}
