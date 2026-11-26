use std::{sync::Arc, time::Duration};

use pas_hq_app::{
    domain::{
        contracts::{
            caching::{Cache, CacheKind},
            utility::Logger,
        },
        models::{
            AppContext,
            logger::{Level, Meta},
        },
    },
    infrastructure::caching::RedisCache,
};

struct NoopLogger;

impl Logger for NoopLogger {
    fn log(
        &self,
        _context: &AppContext,
        _level: Level,
        _tag: &str,
        _message: &str,
        _meta: Option<&Meta>,
    ) {
    }
}

#[tokio::test]
#[ignore = "requires PAS_HQ_APP_TEST_REDIS_URL"]
async fn redis_cache_round_trips_values() {
    let url =
        std::env::var("PAS_HQ_APP_TEST_REDIS_URL").expect("PAS_HQ_APP_TEST_REDIS_URL must be set");
    let cache = RedisCache::new(
        &url,
        Duration::from_secs(60),
        Duration::from_secs(5),
        Arc::new(NoopLogger),
    )
    .await
    .expect("redis connection");
    let context = AppContext::default();
    let key = "pas_hq:test:redis_cache";

    cache
        .set(&context, key, CacheKind::Individual, b"value".to_vec())
        .await;
    assert_eq!(cache.get(&context, key).await, Some(b"value".to_vec()));

    cache.remove(&context, key).await;
    assert!(cache.get(&context, key).await.is_none());
}
