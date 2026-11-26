use std::sync::Arc;

use pas_hq_app::{
    domain::{
        contracts::{utility::Logger, validator::AuthValidator},
        models::{
            ValidatorError,
            logger::{Level, MetaValue},
        },
    },
    infrastructure::utility::validator::RegexAuthValidator,
};

use super::{CapturingLogger, context};

fn validator() -> (Arc<CapturingLogger>, RegexAuthValidator) {
    let logger = Arc::new(CapturingLogger::default());
    let validator = RegexAuthValidator::new(Arc::clone(&logger) as Arc<dyn Logger>);
    (logger, validator)
}

#[test]
fn valid_credentials_are_accepted_without_logging() {
    let (logger, validator) = validator();
    let context = context();

    assert_eq!(validator.username(&context, "valid_user"), Ok(()));
    assert_eq!(validator.password(&context, "Str0ng!Pass"), Ok(()));
    assert!(logger.records().is_empty());
}

#[test]
fn username_rules_are_enforced() {
    let (_, validator) = validator();
    let context = context();

    assert_eq!(
        validator.username(&context, "ab"),
        Err(ValidatorError::UsernameTooShort)
    );
    assert_eq!(
        validator.username(&context, &"a".repeat(33)),
        Err(ValidatorError::UsernameTooLong)
    );
    assert_eq!(
        validator.username(&context, "bad name"),
        Err(ValidatorError::UsernameInvalid)
    );
}

#[test]
fn password_rules_are_enforced() {
    let (_, validator) = validator();
    let context = context();

    assert_eq!(
        validator.password(&context, "Sh0rt!"),
        Err(ValidatorError::PasswordTooShort)
    );
    assert_eq!(
        validator.password(&context, &format!("{}aA1!", "x".repeat(70))),
        Err(ValidatorError::PasswordTooLong)
    );
    assert_eq!(
        validator.password(&context, "lowercase1!"),
        Err(ValidatorError::PasswordMissingUppercase)
    );
    assert_eq!(
        validator.password(&context, "UPPERCASE1!"),
        Err(ValidatorError::PasswordMissingLowercase)
    );
    assert_eq!(
        validator.password(&context, "NoDigits!!"),
        Err(ValidatorError::PasswordMissingDigit)
    );
    assert_eq!(
        validator.password(&context, "NoSymbols1"),
        Err(ValidatorError::PasswordMissingSymbol)
    );
}

#[test]
fn rejection_logs_once_with_classified_metadata() {
    let (logger, validator) = validator();
    let context = context();

    assert_eq!(
        validator.username(&context, "ab"),
        Err(ValidatorError::UsernameTooShort)
    );

    let records = logger.records();
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.level, Level::Error);
    assert_eq!(record.tag, "validator/auth/username");
    assert_eq!(record.message, "username is too short");
    let meta = record.meta.as_ref().unwrap();
    assert_eq!(
        meta["error_code"],
        MetaValue::String("AUTH_USERNAME_TOO_SHORT".into())
    );
}
