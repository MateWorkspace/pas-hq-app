use std::time::{Duration, Instant};

use moka::{policy::Expiry, sync::Cache as Moka};

use crate::domain::{
    contracts::caching::{Cache, CacheFuture, CacheKind},
    models::AppContext,
};

#[derive(Clone)]
struct CacheEntry {
    kind: CacheKind,
    value: Vec<u8>,
}

struct EntryExpiry {
    individual: Duration,
    list: Duration,
}

impl EntryExpiry {
    fn ttl(&self, kind: CacheKind) -> Duration {
        match kind {
            CacheKind::Individual => self.individual,
            CacheKind::List => self.list,
        }
    }
}

impl Expiry<String, CacheEntry> for EntryExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &CacheEntry,
        _created_at: Instant,
    ) -> Option<Duration> {
        Some(self.ttl(value.kind))
    }

    fn expire_after_update(
        &self,
        _key: &String,
        value: &CacheEntry,
        _updated_at: Instant,
        _duration_until_expiry: Option<Duration>,
    ) -> Option<Duration> {
        Some(self.ttl(value.kind))
    }
}

/// Bounded, process-local byte cache with distinct individual and list TTLs.
///
/// Moka performs TinyLFU eviction and per-entry expiration internally. Operations
/// are in-memory and effectively synchronous; the async contract only exists so a
/// Redis adapter can be swapped in without changing callers.
pub struct MokaCache {
    entries: Moka<String, CacheEntry>,
}

impl MokaCache {
    pub fn new(max_capacity: u64, individual_ttl: Duration, list_ttl: Duration) -> Self {
        let entries = Moka::builder()
            .max_capacity(max_capacity)
            .expire_after(EntryExpiry {
                individual: individual_ttl,
                list: list_ttl,
            })
            .build();

        Self { entries }
    }
}

impl Cache for MokaCache {
    fn get<'a>(
        &'a self,
        _context: &'a AppContext,
        key: &'a str,
    ) -> CacheFuture<'a, Option<Vec<u8>>> {
        Box::pin(async move { self.entries.get(key).map(|entry| entry.value) })
    }

    fn set<'a>(
        &'a self,
        _context: &'a AppContext,
        key: &'a str,
        kind: CacheKind,
        value: Vec<u8>,
    ) -> CacheFuture<'a, ()> {
        let key = key.to_owned();
        Box::pin(async move {
            self.entries.insert(key, CacheEntry { kind, value });
        })
    }

    fn remove<'a>(&'a self, _context: &'a AppContext, key: &'a str) -> CacheFuture<'a, ()> {
        let key = key.to_owned();
        Box::pin(async move {
            self.entries.invalidate(&key);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiry_uses_configured_ttls_per_kind() {
        let expiry = EntryExpiry {
            individual: Duration::from_secs(60),
            list: Duration::from_secs(5),
        };
        let individual = CacheEntry {
            kind: CacheKind::Individual,
            value: vec![1_u8],
        };
        let list = CacheEntry {
            kind: CacheKind::List,
            value: vec![1_u8, 2],
        };
        let now = Instant::now();
        let key = "key".to_owned();

        assert_eq!(
            expiry.expire_after_create(&key, &individual, now),
            Some(Duration::from_secs(60))
        );
        assert_eq!(
            expiry.expire_after_create(&key, &list, now),
            Some(Duration::from_secs(5))
        );
    }
}
