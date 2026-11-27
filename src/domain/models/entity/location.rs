use std::{error::Error, fmt, str::FromStr};

use chrono::{DateTime, Utc};

/// A physical site that holds stock.
///
/// Only `PartialEq` is derived: the coordinates are `f64`, which is not `Eq`.
#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub id: i64,
    pub kind: LocationKind,
    pub name: String,
    pub description: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}

/// The role a location plays.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LocationKind {
    Store,
    Warehouse,
}

impl LocationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Store => "store",
            Self::Warehouse => "warehouse",
        }
    }
}

impl fmt::Display for LocationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LocationKind {
    type Err = ParseLocationKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "store" => Ok(Self::Store),
            "warehouse" => Ok(Self::Warehouse),
            _ => Err(ParseLocationKindError),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseLocationKindError;

impl fmt::Display for ParseLocationKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("location kind must be store or warehouse")
    }
}

impl Error for ParseLocationKindError {}
