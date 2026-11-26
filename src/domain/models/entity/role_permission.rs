use chrono::{DateTime, Utc};

/// A grant of one permission to one role.
///
/// The role/permission pair is unique and immutable once created.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolePermission {
    pub id: i64,
    pub role_id: i64,
    pub permission_id: i64,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<i64>,
}
