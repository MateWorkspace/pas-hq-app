use crate::domain::models::AppContext;

use super::{CacheFuture, CacheKind};

/// A best-effort cache for read-through values.
///
/// Values are opaque bytes so an out-of-process adapter (Redis) and an in-process
/// adapter (Moka) are interchangeable behind one contract. Missing, expired, and
/// mismatched entries are all misses, and cache failures are never part of the
/// domain error vocabulary: adapters are expected to swallow them. Callers must
/// not cache data read inside an active transaction.
pub trait Cache: Send + Sync {
    /// Return a cached value. Absent and expired entries both return `None`.
    fn get<'a>(&'a self, context: &'a AppContext, key: &'a str)
    -> CacheFuture<'a, Option<Vec<u8>>>;

    /// Insert or replace a value. The adapter applies the TTL for `kind`.
    fn set<'a>(
        &'a self,
        context: &'a AppContext,
        key: &'a str,
        kind: CacheKind,
        value: Vec<u8>,
    ) -> CacheFuture<'a, ()>;

    /// Remove an exact key. Missing keys are ignored.
    fn remove<'a>(&'a self, context: &'a AppContext, key: &'a str) -> CacheFuture<'a, ()>;
}
