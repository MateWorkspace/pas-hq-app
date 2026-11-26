use crate::domain::{
    contracts::utility::Logger,
    models::{AppContext, ValidatorError, logger::Meta},
};

pub(crate) const USERNAME_PATTERN: &str = r"\A[A-Za-z0-9._-]+\z";
pub(crate) const SLUG_PATTERN: &str = r"\A[a-z0-9]+(?:-[a-z0-9]+)*\z";

pub(crate) const USERNAME_MIN_BYTES: usize = 3;
pub(crate) const USERNAME_MAX_BYTES: usize = 32;
pub(crate) const PASSWORD_MIN_BYTES: usize = 8;
pub(crate) const PASSWORD_MAX_BYTES: usize = 72;
pub(crate) const NAME_MIN_CHARS: usize = 3;
pub(crate) const NAME_MAX_CHARS: usize = 100;
pub(crate) const SLUG_MIN_CHARS: usize = 2;
pub(crate) const SLUG_MAX_CHARS: usize = 64;
pub(crate) const DESCRIPTION_MAX_CHARS: usize = 500;
pub(crate) const BIO_MAX_CHARS: usize = 1_000;

pub(crate) fn has_uppercase(value: &str) -> bool {
    value
        .chars()
        .any(|character| character.is_ascii_uppercase())
}

pub(crate) fn has_lowercase(value: &str) -> bool {
    value
        .chars()
        .any(|character| character.is_ascii_lowercase())
}

pub(crate) fn has_digit(value: &str) -> bool {
    value.chars().any(|character| character.is_ascii_digit())
}

pub(crate) fn has_symbol(value: &str) -> bool {
    value.chars().any(|character| {
        !character.is_alphanumeric() && !character.is_whitespace() && !character.is_control()
    })
}

pub(crate) fn reject(
    logger: &dyn Logger,
    context: &AppContext,
    tag: &str,
    error: ValidatorError,
) -> Result<(), ValidatorError> {
    let message = error.to_string();
    let meta = Meta::from([
        ("error_code".into(), error.code().into()),
        ("reason".into(), message.clone().into()),
    ]);
    logger.error(context, tag, &message, Some(&meta));
    Err(error)
}
