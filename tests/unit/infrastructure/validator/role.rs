use std::sync::Arc;

use pas_hq_app::{
    domain::{
        contracts::{utility::Logger, validator::RoleValidator},
        models::ValidatorError,
    },
    infrastructure::utility::validator::RegexRoleValidator,
};

use super::{CapturingLogger, context};

fn validator() -> (Arc<CapturingLogger>, RegexRoleValidator) {
    let logger = Arc::new(CapturingLogger::default());
    let validator = RegexRoleValidator::new(Arc::clone(&logger) as Arc<dyn Logger>);
    (logger, validator)
}

#[test]
fn valid_role_fields_are_accepted_without_logging() {
    let (logger, validator) = validator();
    let context = context();

    assert_eq!(validator.slug(&context, "admin-role"), Ok(()));
    assert_eq!(validator.name(&context, "Administrator"), Ok(()));
    assert_eq!(validator.description(&context, ""), Ok(()));
    assert!(logger.records().is_empty());
}

#[test]
fn role_field_rules_are_enforced() {
    let (_, validator) = validator();
    let context = context();

    assert_eq!(
        validator.slug(&context, "a"),
        Err(ValidatorError::RoleSlugTooShort)
    );
    assert_eq!(
        validator.slug(&context, &"a".repeat(65)),
        Err(ValidatorError::RoleSlugTooLong)
    );
    assert_eq!(
        validator.slug(&context, "Admin"),
        Err(ValidatorError::RoleSlugInvalid)
    );
    assert_eq!(
        validator.name(&context, "ab"),
        Err(ValidatorError::RoleNameTooShort)
    );
    assert_eq!(
        validator.name(&context, &"a".repeat(101)),
        Err(ValidatorError::RoleNameTooLong)
    );
    assert_eq!(
        validator.description(&context, &"a".repeat(501)),
        Err(ValidatorError::RoleDescriptionTooLong)
    );
}
