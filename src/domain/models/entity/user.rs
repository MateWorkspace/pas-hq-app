use chrono::{DateTime, Utc};

/// An application identity with exactly one role.
///
/// Authentication credentials are stored separately in `BaseAuth`. `is_active`
/// gates whether the account may authenticate; a suspension sets
/// `suspended_until` without deactivating the account.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub role_id: i64,
    pub name: String,
    pub bio: String,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub suspended_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
