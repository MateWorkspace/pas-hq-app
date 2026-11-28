use std::sync::Arc;

use crate::domain::{
    contracts::{utility::Logger, validator::ApiKeyValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{DESCRIPTION_MAX_CHARS, NAME_MAX_CHARS, NAME_MIN_CHARS, reject};

/// Field rules for API key metadata.
pub struct RegexApiKeyValidator {
    logger: Arc<dyn Logger>,
}

impl RegexApiKeyValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl ApiKeyValidator for RegexApiKeyValidator {
    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if value.trim().is_empty() || chars < NAME_MIN_CHARS {
            Some(ValidatorError::ApiKeyNameTooShort)
        } else if chars > NAME_MAX_CHARS {
            Some(ValidatorError::ApiKeyNameTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/api_key/name",
                error,
            ),
            None => Ok(()),
        }
    }

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let error = if value.chars().count() > DESCRIPTION_MAX_CHARS {
            Some(ValidatorError::ApiKeyDescriptionTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/api_key/description",
                error,
            ),
            None => Ok(()),
        }
    }
}
