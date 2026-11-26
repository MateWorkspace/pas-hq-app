use pas_hq_app::domain::models::{PasswordError, TokenError};

#[test]
fn password_error_codes_are_stable() {
    assert_eq!(PasswordError::Mismatch.code(), "PASSWORD_MISMATCH");
    assert_eq!(PasswordError::HashFailed.code(), "PASSWORD_HASH_FAILED");
    assert_eq!(
        PasswordError::CompareFailed.code(),
        "PASSWORD_COMPARE_FAILED"
    );
}

#[test]
fn token_error_codes_are_stable() {
    assert_eq!(TokenError::BadArgs.code(), "BAD_ARGS");
    assert_eq!(TokenError::Failed.code(), "TOKEN_FAILED");
    assert_eq!(TokenError::Expired.code(), "TOKEN_EXPIRED");
    assert_eq!(TokenError::Invalid.code(), "TOKEN_INVALID");
}
