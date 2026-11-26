use std::sync::Arc;

use regex::Regex;

use crate::domain::{
    contracts::{utility::Logger, validator::AuthValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{
    PASSWORD_MAX_BYTES, PASSWORD_MIN_BYTES, USERNAME_MAX_BYTES, USERNAME_MIN_BYTES,
    USERNAME_PATTERN, has_digit, has_lowercase, has_symbol, has_uppercase, reject,
};

/// Credential rules for base authentication records.
///
/// Passwords require an uppercase letter, a lowercase letter, a digit, and a
/// symbol, in addition to their length bound. Values are never retained or logged.
pub struct RegexAuthValidator {
    logger: Arc<dyn Logger>,
    username_pattern: Regex,
}

impl RegexAuthValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            logger,
            username_pattern: Regex::new(USERNAME_PATTERN).expect("username pattern is valid"),
        }
    }
}

impl AuthValidator for RegexAuthValidator {
    fn username(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let bytes = value.len();
        let error = if bytes < USERNAME_MIN_BYTES {
            Some(ValidatorError::UsernameTooShort)
        } else if bytes > USERNAME_MAX_BYTES {
            Some(ValidatorError::UsernameTooLong)
        } else if !self.username_pattern.is_match(value) {
            Some(ValidatorError::UsernameInvalid)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/auth/username",
                error,
            ),
            None => Ok(()),
        }
    }

    fn password(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        let bytes = value.len();
        let error = if bytes < PASSWORD_MIN_BYTES {
            Some(ValidatorError::PasswordTooShort)
        } else if bytes > PASSWORD_MAX_BYTES {
            Some(ValidatorError::PasswordTooLong)
        } else if !has_uppercase(value) {
            Some(ValidatorError::PasswordMissingUppercase)
        } else if !has_lowercase(value) {
            Some(ValidatorError::PasswordMissingLowercase)
        } else if !has_digit(value) {
            Some(ValidatorError::PasswordMissingDigit)
        } else if !has_symbol(value) {
            Some(ValidatorError::PasswordMissingSymbol)
        } else {
            None
        };

        match error {
            Some(error) => reject(
                self.logger.as_ref(),
                context,
                "validator/auth/password",
                error,
            ),
            None => Ok(()),
        }
    }
}
