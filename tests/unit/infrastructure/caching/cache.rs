use std::time::Duration;

use pas_hq_app::{
    domain::{
        contracts::caching::{Cache, CacheKind},
        models::AppContext,
    },
    infrastructure::caching::MokaCache,
};

#[tokio::test]
async fn moka_cache_stores_overwrites_and_removes_values() {
    let cache = MokaCache::new(16, Duration::from_secs(60), Duration::from_secs(5));
    let context = AppContext::default();

    assert!(cache.get(&context, "key").await.is_none());

    cache
        .set(&context, "key", CacheKind::Individual, b"first".to_vec())
        .await;
    assert_eq!(cache.get(&context, "key").await, Some(b"first".to_vec()));

    cache
        .set(&context, "key", CacheKind::List, b"second".to_vec())
        .await;
    assert_eq!(cache.get(&context, "key").await, Some(b"second".to_vec()));

    cache.remove(&context, "key").await;
    assert!(cache.get(&context, "key").await.is_none());
}
