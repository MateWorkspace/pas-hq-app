use chrono::{DateTime, Utc};

/// The current stock of one SKU at one location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inventory {
    pub id: i64,
    pub location_id: i64,
    pub sku_id: i64,
    pub stock: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
