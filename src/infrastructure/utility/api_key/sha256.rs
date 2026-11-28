use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

use crate::domain::{
    contracts::utility::api_key::{ApiKey, GeneratedApiKey},
    models::ApiKeyError,
};

/// Prefix every issued key starts with.
pub(crate) const KEY_PREFIX: &str = "pas-";

/// Number of random bytes behind the prefix: 43 URL-safe characters.
const KEY_RANDOM_BYTES: usize = 32;

/// Characters kept at each end of the redacted preview.
const REDACT_PREFIX_CHARS: usize = 8;
const REDACT_SUFFIX_CHARS: usize = 4;

/// API key adapter using cryptographic randomness and a SHA-256 digest.
///
/// The digest is deterministic so the same raw key always maps to one database row
/// and one cache entry. Keys are high-entropy, so a fast digest is the correct
/// choice here rather than a salted password hash.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sha256ApiKey;

impl Sha256ApiKey {
    pub const fn new() -> Self {
        Self
    }
}

impl ApiKey for Sha256ApiKey {
    fn generate(&self) -> Result<GeneratedApiKey, ApiKeyError> {
        let mut bytes = [0_u8; KEY_RANDOM_BYTES];
        getrandom::fill(&mut bytes).map_err(|_| ApiKeyError::GenerationFailed)?;

        let raw = format!("{KEY_PREFIX}{}", URL_SAFE_NO_PAD.encode(bytes));
        let hash = self.hash(&raw)?;
        let redacted = redact(&raw)?;

        Ok(GeneratedApiKey {
            raw,
            hash,
            redacted,
        })
    }

    fn hash(&self, raw: &str) -> Result<String, ApiKeyError> {
        if raw.is_empty() {
            return Err(ApiKeyError::HashFailed);
        }

        let digest = Sha256::digest(raw.as_bytes());

        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }

    fn is_well_formed(&self, raw: &str) -> bool {
        let Some(body) = raw.strip_prefix(KEY_PREFIX) else {
            return false;
        };

        body.len() == 43
            && body.chars().all(|character| {
                character.is_ascii_alphanumeric() || character == '-' || character == '_'
            })
    }
}

fn redact(raw: &str) -> Result<String, ApiKeyError> {
    if raw.chars().count() <= REDACT_PREFIX_CHARS + REDACT_SUFFIX_CHARS {
        return Err(ApiKeyError::GenerationFailed);
    }

    let prefix: String = raw.chars().take(REDACT_PREFIX_CHARS).collect();
    let suffix: String = raw
        .chars()
        .rev()
        .take(REDACT_SUFFIX_CHARS)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    Ok(format!("{prefix}\u{2026}{suffix}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_keys_are_prefixed_and_unique() {
        let adapter = Sha256ApiKey;
        let first = adapter.generate().unwrap();
        let second = adapter.generate().unwrap();

        assert!(first.raw.starts_with(KEY_PREFIX));
        assert!(adapter.is_well_formed(&first.raw));
        assert_ne!(first.raw, second.raw);
        assert_ne!(first.hash, second.hash);
    }

    #[test]
    fn hashing_is_deterministic_and_matches_generation() {
        let adapter = Sha256ApiKey;
        let generated = adapter.generate().unwrap();

        assert_eq!(adapter.hash(&generated.raw).unwrap(), generated.hash);
        assert_eq!(generated.hash.len(), 64);
    }

    #[test]
    fn redaction_keeps_only_the_ends() {
        let adapter = Sha256ApiKey;
        let generated = adapter.generate().unwrap();

        assert!(generated.redacted.starts_with("pas-"));
        assert!(
            generated
                .redacted
                .ends_with(&generated.raw[generated.raw.len() - 4..])
        );
        assert!(!generated.redacted.contains(&generated.raw[10..20]));
    }

    #[test]
    fn malformed_keys_are_rejected() {
        let adapter = Sha256ApiKey;

        assert!(!adapter.is_well_formed(""));
        assert!(!adapter.is_well_formed("pas-"));
        assert!(!adapter.is_well_formed("xyz-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(!adapter.is_well_formed("pas-short"));
    }
}
