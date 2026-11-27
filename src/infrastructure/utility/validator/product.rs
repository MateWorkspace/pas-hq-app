use std::sync::Arc;

use crate::domain::{
    contracts::{utility::Logger, validator::ProductValidator},
    models::{AppContext, ValidatorError},
};

use super::shared::{DESCRIPTION_MAX_CHARS, reject};

/// Field rules for product models.
pub struct RegexProductValidator {
    logger: Arc<dyn Logger>,
}

impl RegexProductValidator {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self { logger }
    }

    fn non_blank(
        &self,
        context: &AppContext,
        tag: &str,
        value: &str,
        error: ValidatorError,
    ) -> Result<(), ValidatorError> {
        if value.trim().is_empty() {
            reject(self.logger.as_ref(), context, tag, error)
        } else {
            Ok(())
        }
    }
}

impl ProductValidator for RegexProductValidator {
    fn brand(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        self.non_blank(
            context,
            "validator/product/brand",
            value,
            ValidatorError::ProductBrandInvalid,
        )
    }

    fn model(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        self.non_blank(
            context,
            "validator/product/model",
            value,
            ValidatorError::ProductModelInvalid,
        )
    }

    fn manufacturer(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        self.non_blank(
            context,
            "validator/product/manufacturer",
            value,
            ValidatorError::ProductManufacturerInvalid,
        )
    }

    fn description(&self, context: &AppContext, value: &str) -> Result<(), ValidatorError> {
        if value.chars().count() > DESCRIPTION_MAX_CHARS {
            reject(
                self.logger.as_ref(),
                context,
                "validator/product/description",
                ValidatorError::ProductDescriptionTooLong,
            )
        } else {
            Ok(())
        }
    }
}
