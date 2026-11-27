use chrono::{DateTime, Utc};

/// A variant dimension of a product, such as Color or Storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductVariant {
    pub id: i64,
    pub product_id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
