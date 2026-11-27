use crate::domain::models::{AppContext, ValidatorError};

/// Field checks for stock-holding sites.
///
/// The name is a non-blank label and the description may be empty. Implementations
/// do not normalize or retain input.
pub trait LocationValidator: Send + Sync {
    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;
}
