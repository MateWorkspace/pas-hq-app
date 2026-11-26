use crate::domain::models::PasswordError;

/// Classic-credential password hashing and verification.
///
/// Implementations must not retain, log, or normalize raw passwords and must never
/// truncate input silently. A mismatch is classified separately from a malformed
/// stored hash or a hashing failure.
pub trait Password: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, PasswordError>;

    fn compare(&self, stored_hash: &str, password: &str) -> Result<(), PasswordError>;
}
