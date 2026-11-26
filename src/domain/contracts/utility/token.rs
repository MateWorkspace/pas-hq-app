use crate::domain::models::{AccessClaims, RefreshClaims, TokenError};

/// Signed access and refresh tokens for user sessions.
///
/// Generation embeds the identity supplied by the caller. Validation rechecks the
/// signature, token type, and timestamps, but never reads live identities, grants,
/// or revocation state: callers must recheck those before trusting refreshed claims.
pub trait Token: Send + Sync {
    fn generate_access(&self, claims: &AccessClaims) -> Result<String, TokenError>;

    fn validate_access(&self, token: &str) -> Result<AccessClaims, TokenError>;

    fn generate_refresh(&self, claims: &RefreshClaims) -> Result<String, TokenError>;

    fn validate_refresh(&self, token: &str) -> Result<RefreshClaims, TokenError>;
}
