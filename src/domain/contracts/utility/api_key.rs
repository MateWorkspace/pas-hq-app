use crate::domain::models::ApiKeyError;

/// Generated key material. `raw` is shown to the owner exactly once and is never
/// stored; only `hash` and `redacted` are persisted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedApiKey {
    pub raw: String,
    pub hash: String,
    pub redacted: String,
}

/// Long-lived API key generation and hashing.
///
/// Keys are high-entropy random values, so a fast deterministic digest is correct
/// here: the hash is the lookup key and the cache key. Implementations must never
/// log or retain a raw key.
pub trait ApiKey: Send + Sync {
    /// Create a new key with the service prefix.
    fn generate(&self) -> Result<GeneratedApiKey, ApiKeyError>;

    /// Deterministic digest of a raw key, used for lookup and cache identity.
    fn hash(&self, raw: &str) -> Result<String, ApiKeyError>;

    /// True when the value has the expected prefix and length shape.
    fn is_well_formed(&self, raw: &str) -> bool;
}
