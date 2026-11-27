use crate::domain::models::{AppContext, ValidatorError};

/// Field checks for product models.
///
/// Brand, model, and manufacturer are non-blank labels; the description may be
/// empty. Implementations do not normalize or retain input.
pub trait ProductValidator: Send + Sync {
    fn brand(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn model(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn manufacturer(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError>;
}
