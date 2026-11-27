use chrono::{DateTime, Utc};

/// A selectable value of a variant dimension, such as Black or 256GB.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductVariantSelection {
    pub id: i64,
    pub product_variant_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
