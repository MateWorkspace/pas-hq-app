use chrono::{DateTime, Utc};
use serde::Serialize;

/// Timestamps are always rendered as RFC 3339 in UTC.
#[derive(Clone, Debug, Serialize)]
pub struct AuditResponse {
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl AuditResponse {
    pub(crate) fn new(created_at: DateTime<Utc>, updated_at: Option<DateTime<Utc>>) -> Self {
        Self {
            created_at: created_at.to_rfc3339(),
            updated_at: updated_at.map(|value| value.to_rfc3339()),
        }
    }
}
