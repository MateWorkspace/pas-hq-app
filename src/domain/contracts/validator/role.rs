use crate::domain::models::{AppContext, ValidatorError};

/// Field checks for role identities.
///
/// The slug is a stable lowercase identifier, the name is a non-blank label, and
/// the description may be empty. Implementations do not normalize or retain input.
pub trait RoleValidator: Send + Sync {
    fn slug(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;
}
