use std::{error::Error, fmt, str::FromStr};

use chrono::{DateTime, Utc};

/// An append-only record of stock moving in, out, or between locations.
///
/// `quantity` is a positive magnitude; direction comes from the locations. The
/// name snapshots keep the row readable after its SKU or location is deleted. The
/// record carries no update audit fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StockMovement {
    pub id: i64,
    pub sku_id: Option<i64>,
    pub sku_label: Option<String>,
    pub src_location_id: Option<i64>,
    pub src_location_name: Option<String>,
    pub dest_location_id: Option<i64>,
    pub dest_location_name: Option<String>,
    pub quantity: i64,
    pub reason: MovementReason,
    pub created_at: DateTime<Utc>,
}

/// Why stock moved. Direction and pairing are validated by the application.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MovementReason {
    Receipt,
    Sale,
    Transfer,
    Adjustment,
    WriteOff,
    Return,
}

impl MovementReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Receipt => "receipt",
            Self::Sale => "sale",
            Self::Transfer => "transfer",
            Self::Adjustment => "adjustment",
            Self::WriteOff => "write_off",
            Self::Return => "return",
        }
    }
}

impl fmt::Display for MovementReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MovementReason {
    type Err = ParseMovementReasonError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "receipt" => Ok(Self::Receipt),
            "sale" => Ok(Self::Sale),
            "transfer" => Ok(Self::Transfer),
            "adjustment" => Ok(Self::Adjustment),
            "write_off" => Ok(Self::WriteOff),
            "return" => Ok(Self::Return),
            _ => Err(ParseMovementReasonError),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseMovementReasonError;

impl fmt::Display for ParseMovementReasonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "movement reason must be receipt, sale, transfer, adjustment, write_off, or return",
        )
    }
}

impl Error for ParseMovementReasonError {}
