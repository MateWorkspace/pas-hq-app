use std::sync::Arc;

use pas_hq_app::{
    domain::{
        contracts::{utility::Logger, validator::UserValidator},
        models::ValidatorError,
    },
    infrastructure::utility::validator::RegexUserValidator,
};

use super::{CapturingLogger, context};

fn validator() -> (Arc<CapturingLogger>, RegexUserValidator) {
    let logger = Arc::new(CapturingLogger::default());
    let validator = RegexUserValidator::new(Arc::clone(&logger) as Arc<dyn Logger>);
    (logger, validator)
}

#[test]
fn valid_user_fields_are_accepted_without_logging() {
    let (logger, validator) = validator();
    let context = context();

    assert_eq!(validator.name(&context, "Ada Lovelace"), Ok(()));
    assert_eq!(validator.bio(&context, ""), Ok(()));
    assert!(logger.records().is_empty());
}

#[test]
fn user_field_rules_are_enforced() {
    let (_, validator) = validator();
    let context = context();

    assert_eq!(
        validator.name(&context, ""),
        Err(ValidatorError::UserNameTooShort)
    );
    assert_eq!(
        validator.name(&context, &"a".repeat(101)),
        Err(ValidatorError::UserNameTooLong)
    );
    assert_eq!(
        validator.bio(&context, &"a".repeat(1_001)),
        Err(ValidatorError::UserBioTooLong)
    );
    assert_eq!(validator.bio(&context, &"a".repeat(1_000)), Ok(()));
}
