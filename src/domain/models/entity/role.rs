use chrono::{DateTime, Utc};

/// A role identity with a stable slug and optional default status.
///
/// At most one role is the default; the partial unique index documented in the
/// ERD enforces it. The slug is reserved and immutable once created.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Role {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
