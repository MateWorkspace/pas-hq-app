use crate::domain::models::{AccessClaims, AppContext};

use super::CacheFuture;

/// Caches resolved claims by API key digest.
///
/// The key is the SHA-256 digest, so entries are exact and can be removed on
/// rotation, disable, or deletion without scanning. Failures are best-effort and
/// never change the caller's result.
pub trait ApiKeyClaimsCache: Send + Sync {
    fn get<'a>(
        &'a self,
        context: &'a AppContext,
        key_hash: &'a str,
    ) -> CacheFuture<'a, Option<AccessClaims>>;

    fn set<'a>(
        &'a self,
        context: &'a AppContext,
        key_hash: &'a str,
        claims: &'a AccessClaims,
    ) -> CacheFuture<'a, ()>;

    fn remove<'a>(&'a self, context: &'a AppContext, key_hash: &'a str) -> CacheFuture<'a, ()>;
}
