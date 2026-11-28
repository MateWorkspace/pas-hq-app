use crate::domain::models::{AppContext, ValidatorError};

/// Field checks for API key metadata.
///
/// The name is a non-blank label unique per user; the description may be empty.
/// Implementations do not normalize or retain input and never see key material.
pub trait ApiKeyValidator: Send + Sync {
    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;
}
