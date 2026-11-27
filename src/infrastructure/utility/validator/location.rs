use std::sync::Arc;

use crate::domain::{
    contracts::{utility::Logger, validator::LocationValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{DESCRIPTION_MAX_CHARS, NAME_MAX_CHARS, NAME_MIN_CHARS, reject};

/// Field rules for stock-holding sites.
pub struct RegexLocationValidator {
    logger: Arc<dyn Logger>,
}

impl RegexLocationValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl LocationValidator for RegexLocationValidator {
    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if value.trim().is_empty() || chars < NAME_MIN_CHARS {
            Some(ValidatorError::LocationNameTooShort)
        } else if chars > NAME_MAX_CHARS {
            Some(ValidatorError::LocationNameTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/location/name",
                error,
            ),
            None => Ok(()),
        }
    }

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let error = if value.chars().count() > DESCRIPTION_MAX_CHARS {
            Some(ValidatorError::LocationDescriptionTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/location/description",
                error,
            ),
            None => Ok(()),
        }
    }
}
