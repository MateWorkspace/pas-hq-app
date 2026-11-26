use chrono::{DateTime, Utc};

/// A stable authorization action referenced by role grants.
///
/// Permissions are created only; they are never updated or removed, so the slug
/// is never reused.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permission {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
