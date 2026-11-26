use std::{error::Error as StdError, fmt};

/// Password hashing and comparison failures with stable diagnostic codes.
///
/// A credential mismatch is kept distinct from hashing failures and malformed
/// stored hashes. Variants carry no raw password or native driver text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PasswordError {
    Mismatch,
    HashFailed,
    CompareFailed,
}

impl PasswordError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Mismatch => "PASSWORD_MISMATCH",
            Self::HashFailed => "PASSWORD_HASH_FAILED",
            Self::CompareFailed => "PASSWORD_COMPARE_FAILED",
        }
    }
}

impl fmt::Display for PasswordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mismatch => formatter.write_str("password does not match"),
            Self::HashFailed => formatter.write_str("failed to hash password"),
            Self::CompareFailed => formatter.write_str("failed to compare password"),
        }
    }
}

impl StdError for PasswordError {}
