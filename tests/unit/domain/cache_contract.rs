use pas_hq_app::domain::contracts::caching::{Cache, CacheKind};

#[test]
fn cache_contract_is_object_safe() {
    fn accept(_: &dyn Cache) {}

    let _ = accept;
}

#[test]
fn cache_kinds_are_distinct() {
    assert_ne!(CacheKind::Individual, CacheKind::List);
}
