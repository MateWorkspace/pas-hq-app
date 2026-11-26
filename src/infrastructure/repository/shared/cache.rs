use std::sync::atomic::{AtomicU64, Ordering};

/// Versioned cache keys and invalidation state for one repository namespace.
///
/// A successful write advances the generation, making earlier keys unreachable
/// without a scan; their entries expire through the adapter TTL and capacity
/// policy. Invalidation is process-local, so a multi-instance deployment relies on
/// the TTL to bound staleness.
pub(crate) struct CacheNamespace {
    entity: &'static str,
    generation: AtomicU64,
}

impl CacheNamespace {
    pub(crate) fn new(entity: &'static str) -> Self {
        Self {
            entity,
            generation: AtomicU64::new(0),
        }
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    pub(crate) fn invalidation_guard(&self) -> InvalidationGuard<'_> {
        InvalidationGuard { namespace: self }
    }

    pub(crate) fn text_key(&self, generation: u64, kind: &str, value: &str) -> String {
        format!(
            "pas_hq:v1:{}:g{}:{}:{}",
            self.entity, generation, kind, value
        )
    }

    fn invalidate(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
    }
}

/// Invalidates an entity namespace when dropped.
///
/// Keeping this guard across the write future also covers cancellation: a dropped
/// write future still makes cached reads unreachable. Failed writes may invalidate
/// unnecessarily, which is safe.
pub(crate) struct InvalidationGuard<'a> {
    namespace: &'a CacheNamespace,
}

impl Drop for InvalidationGuard<'_> {
    fn drop(&mut self) {
        self.namespace.invalidate();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidation_guard_advances_generation_on_drop() {
        let namespace = CacheNamespace::new("role_permission");
        let before = namespace.generation();

        {
            let _guard = namespace.invalidation_guard();
        }

        assert_eq!(namespace.generation(), before + 1);
    }

    #[test]
    fn keys_are_scoped_by_entity_and_generation() {
        let namespace = CacheNamespace::new("role_permission");
        let key = namespace.text_key(1, "slug", "admin");
        let other = namespace.text_key(2, "slug", "admin");

        assert!(key.contains("role_permission"));
        assert!(key.contains("admin"));
        assert_ne!(key, other);
    }
}
