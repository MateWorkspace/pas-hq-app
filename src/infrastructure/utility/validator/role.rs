use std::sync::Arc;

use regex::Regex;

use crate::domain::{
    contracts::{utility::Logger, validator::RoleValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{
    DESCRIPTION_MAX_CHARS, NAME_MAX_CHARS, NAME_MIN_CHARS, SLUG_MAX_CHARS, SLUG_MIN_CHARS,
    SLUG_PATTERN, reject,
};

/// Field rules for role identities.
pub struct RegexRoleValidator {
    logger: Arc<dyn Logger>,
    slug_pattern: Regex,
}

impl RegexRoleValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            logger,
            slug_pattern: Regex::new(SLUG_PATTERN).expect("slug pattern is valid"),
        }
    }
}

impl RoleValidator for RegexRoleValidator {
    fn slug(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if chars < SLUG_MIN_CHARS {
            Some(ValidatorError::RoleSlugTooShort)
        } else if chars > SLUG_MAX_CHARS {
            Some(ValidatorError::RoleSlugTooLong)
        } else if !self.slug_pattern.is_match(value) {
            Some(ValidatorError::RoleSlugInvalid)
        } else {
            None
        };

        match error {
            Some(error) => reject(self.logger.as_ref(), context, "validator/role/slug", error),
            None => Ok(()),
        }
    }

    fn name(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let chars = value.chars().count();
        let error = if value.trim().is_empty() || chars < NAME_MIN_CHARS {
            Some(ValidatorError::RoleNameTooShort)
        } else if chars > NAME_MAX_CHARS {
            Some(ValidatorError::RoleNameTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(self.logger.as_ref(), context, "validator/role/name", error),
            None => Ok(()),
        }
    }

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let error = if value.chars().count() > DESCRIPTION_MAX_CHARS {
            Some(ValidatorError::RoleDescriptionTooLong)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/role/description",
                error,
            ),
            None => Ok(()),
        }
    }
}
