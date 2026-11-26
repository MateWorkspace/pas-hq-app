use bcrypt::{DEFAULT_COST, hash, verify};

use crate::domain::{contracts::utility::password::Password, models::PasswordError};

/// bcrypt accepts at most 71 significant password bytes plus a terminator.
const MIN_COST: u32 = 4;
const MAX_COST: u32 = 31;
const MAX_PASSWORD_BYTES: usize = 72;

/// Password adapter backed by bcrypt with an explicit cost.
///
/// An out-of-range cost falls back to [`DEFAULT_COST`]. Passwords longer than
/// bcrypt's 72-byte boundary are rejected instead of being truncated, and a
/// mismatch is distinguished from a malformed stored hash.
pub struct BcryptPassword {
    cost: u32,
}

impl BcryptPassword {
    pub fn new(cost: u32) -> Self {
        let cost = if (MIN_COST..=MAX_COST).contains(&cost) {
            cost
        } else {
            DEFAULT_COST
        };

        Self { cost }
    }
}

impl Password for BcryptPassword {
    fn hash(&self, password: &str) -> Result<String, PasswordError> {
        if password.len() > MAX_PASSWORD_BYTES {
            return Err(PasswordError::HashFailed);
        }

        hash(password.as_bytes(), self.cost).map_err(|_| PasswordError::HashFailed)
    }

    fn compare(&self, stored_hash: &str, password: &str) -> Result<(), PasswordError> {
        if password.len() > MAX_PASSWORD_BYTES {
            return Err(PasswordError::CompareFailed);
        }

        match verify(password.as_bytes(), stored_hash) {
            Ok(true) => Ok(()),
            Ok(false) => Err(PasswordError::Mismatch),
            Err(_) => Err(PasswordError::CompareFailed),
        }
    }
}
