use crate::domain::models::{AccessClaims, AppContext, entity::ApiKey};

use super::super::UsecaseFuture;

/// A newly issued key. `raw` is returned exactly once and can never be retrieved
/// again; only `key`, whose digest is stored, is persisted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuedApiKey {
    pub key: ApiKey,
    pub raw: String,
}

/// Validates a long-lived API key into the same claims a token produces.
///
/// A key is a credential, not a scope: it carries the owner's current role, so a
/// role change applies on the next validation. The result is cached by key digest,
/// and every rotation, disable, or deletion removes that entry.
pub trait ApiKeyAuth: Send + Sync {
    /// Reject a malformed, unknown, disabled, or owner-inactive key with
    /// `UNAUTHORIZED`. A well-formed key whose digest is unknown is rejected the
    /// same way, so the caller cannot probe for valid keys.
    fn validate_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        raw_key: &'a str,
    ) -> UsecaseFuture<'a, AccessClaims>;
}
