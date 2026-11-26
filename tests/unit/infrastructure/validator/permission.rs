use std::sync::Arc;

use pas_hq_app::{
    domain::{
        contracts::{utility::Logger, validator::PermissionValidator},
        models::ValidatorError,
    },
    infrastructure::utility::validator::RegexPermissionValidator,
};

use super::{CapturingLogger, context};

fn validator() -> (Arc<CapturingLogger>, RegexPermissionValidator) {
    let logger = Arc::new(CapturingLogger::default());
    let validator = RegexPermissionValidator::new(Arc::clone(&logger) as Arc<dyn Logger>);
    (logger, validator)
}

#[test]
fn valid_permission_fields_are_accepted_without_logging() {
    let (logger, validator) = validator();
    let context = context();

    assert_eq!(validator.slug(&context, "user-get"), Ok(()));
    assert_eq!(validator.name(&context, "Get user"), Ok(()));
    assert_eq!(validator.description(&context, ""), Ok(()));
    assert!(logger.records().is_empty());
}

#[test]
fn permission_field_rules_are_enforced() {
    let (_, validator) = validator();
    let context = context();

    assert_eq!(
        validator.slug(&context, "a"),
        Err(ValidatorError::PermissionSlugTooShort)
    );
    assert_eq!(
        validator.slug(&context, &"a".repeat(65)),
        Err(ValidatorError::PermissionSlugTooLong)
    );
    assert_eq!(
        validator.slug(&context, "User_Get"),
        Err(ValidatorError::PermissionSlugInvalid)
    );
    assert_eq!(
        validator.name(&context, "ab"),
        Err(ValidatorError::PermissionNameTooShort)
    );
    assert_eq!(
        validator.name(&context, &"a".repeat(101)),
        Err(ValidatorError::PermissionNameTooLong)
    );
    assert_eq!(
        validator.description(&context, &"a".repeat(501)),
        Err(ValidatorError::PermissionDescriptionTooLong)
    );
}
