use std::sync::Arc;

use crate::domain::{
    contracts::{utility::Logger, validator::UserValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{BIO_MAX_CHARS, NAME_MAX_CHARS, NAME_MIN_CHARS, reject};

/// Field rules for application identities.
pub struct RegexUserValidator {
    logger: Arc<dyn Logger>,
}

impl RegexUserValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl UserValidator for RegexUserValidator {
    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if value.trim().is_empty() || chars < NAME_MIN_CHARS {
            Some(ValidatorError::UserNameTooShort)
        } else if chars > NAME_MAX_CHARS {
            Some(ValidatorError::UserNameTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(self.logger.as_ref(), context, "validator/user/name", error),
            None => Ok(()),
        }
    }

    fn bio(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let error = if value.chars().count() > BIO_MAX_CHARS {
            Some(ValidatorError::UserBioTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(self.logger.as_ref(), context, "validator/user/bio", error),
            None => Ok(()),
        }
    }
}
