use chrono::{DateTime, Utc};

/// A product model identified by brand, model, and manufacturer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product {
    pub id: i64,
    pub brand: String,
    pub model: String,
    pub manufacturer: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
