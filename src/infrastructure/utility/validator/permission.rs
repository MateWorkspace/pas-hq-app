use std::sync::Arc;

use regex::Regex;

use crate::domain::{
    contracts::{utility::Logger, validator::PermissionValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{
    DESCRIPTION_MAX_CHARS, NAME_MAX_CHARS, NAME_MIN_CHARS, SLUG_MAX_CHARS, SLUG_MIN_CHARS,
    SLUG_PATTERN, reject,
};

/// Field rules for permission identities.
pub struct RegexPermissionValidator {
    logger: Arc<dyn Logger>,
    slug_pattern: Regex,
}

impl RegexPermissionValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            logger,
            slug_pattern: Regex::new(SLUG_PATTERN).expect("slug pattern is valid"),
        }
    }
}

impl PermissionValidator for RegexPermissionValidator {
    fn slug(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if chars < SLUG_MIN_CHARS {
            Some(ValidatorError::PermissionSlugTooShort)
        } else if chars > SLUG_MAX_CHARS {
            Some(ValidatorError::PermissionSlugTooLong)
        } else if !self.slug_pattern.is_match(value) {
            Some(ValidatorError::PermissionSlugInvalid)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/permission/slug",
                error,
            ),
            None => Ok(()),
        }
    }

    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if value.trim().is_empty() || chars < NAME_MIN_CHARS {
            Some(ValidatorError::PermissionNameTooShort)
        } else if chars > NAME_MAX_CHARS {
            Some(ValidatorError::PermissionNameTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/permission/name",
                error,
            ),
            None => Ok(()),
        }
    }

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let error = if value.chars().count() > DESCRIPTION_MAX_CHARS {
            Some(ValidatorError::PermissionDescriptionTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/permission/description",
                error,
            ),
            None => Ok(()),
        }
    }
}
