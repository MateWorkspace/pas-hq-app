use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

/// The price of a SKU in one currency.
///
/// A `location_id` of `None` is the global price; `Some` overrides it for that
/// location. Lookups fall back to the global row.
///
/// Only `PartialEq` is derived: `BigDecimal` is not `Eq`.
#[derive(Clone, Debug, PartialEq)]
pub struct Price {
    pub id: i64,
    pub sku_id: i64,
    pub location_id: Option<i64>,
    pub currency: String,
    pub amount: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
