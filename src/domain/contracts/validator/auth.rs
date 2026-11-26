use crate::domain::models::{AppContext, ValidatorError};

/// Credential field checks for base authentication records.
///
/// Implementations must not hash, normalize, log, or retain the supplied value.
/// Failures are classified with stable [`ValidatorError`] codes; valid input does
/// not touch the logger.
pub trait AuthValidator: Send + Sync {
    fn username(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn password(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;
}
