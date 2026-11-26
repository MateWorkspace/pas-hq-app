use crate::domain::models::{AppContext, ValidatorError};

/// Field checks for application identities.
///
/// The name is a non-blank label and the bio may be empty. Implementations do not
/// normalize or retain input.
pub trait UserValidator: Send + Sync {
    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn bio(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;
}
