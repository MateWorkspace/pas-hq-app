use std::{sync::Arc, time::Duration};

use redis::{AsyncCommands, aio::ConnectionManager};

use crate::domain::{
    contracts::{
        caching::{Cache, CacheFuture, CacheKind},
        utility::Logger,
    },
    models::{AppContext, logger::Meta},
};

/// Out-of-process byte cache backed by Redis.
///
/// Each value is stored under the exact key with a per-kind TTL. Connection and
/// command failures are logged and treated as misses or no-ops; they never change
/// the caller's result.
pub struct RedisCache {
    connection: ConnectionManager,
    individual_ttl: Duration,
    list_ttl: Duration,
    logger: Arc<dyn Logger>,
}

impl RedisCache {
    pub async fn new(
        url: &str,
        individual_ttl: Duration,
        list_ttl: Duration,
        logger: Arc<dyn Logger>,
    ) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        let connection = client.get_connection_manager().await?;

        Ok(Self {
            connection,
            individual_ttl,
            list_ttl,
            logger,
        })
    }

    fn ttl_seconds(&self, kind: CacheKind) -> u64 {
        let ttl = match kind {
            CacheKind::Individual => self.individual_ttl,
            CacheKind::List => self.list_ttl,
        };

        ttl.as_secs().max(1)
    }

    fn report_failure(&self, context: &AppContext, tag: &str, message: &str) {
        let meta = Meta::from([("error_code".into(), "CACHE_UNAVAILABLE".into())]);
        self.logger.warn(context, tag, message, Some(&meta));
    }
}

impl Cache for RedisCache {
    fn get<'a>(
        &'a self,
        context: &'a AppContext,
        key: &'a str,
    ) -> CacheFuture<'a, Option<Vec<u8>>> {
        Box::pin(async move {
            let mut connection = self.connection.clone();
            match connection.get::<_, Option<Vec<u8>>>(key).await {
                Ok(value) => value,
                Err(_) => {
                    self.report_failure(context, "cache/redis/get", "cache read failed");
                    None
                }
            }
        })
    }

    fn set<'a>(
        &'a self,
        context: &'a AppContext,
        key: &'a str,
        kind: CacheKind,
        value: Vec<u8>,
    ) -> CacheFuture<'a, ()> {
        Box::pin(async move {
            let mut connection = self.connection.clone();
            if connection
                .set_ex::<_, _, ()>(key, value, self.ttl_seconds(kind))
                .await
                .is_err()
            {
                self.report_failure(context, "cache/redis/set", "cache write failed");
            }
        })
    }

    fn remove<'a>(&'a self, context: &'a AppContext, key: &'a str) -> CacheFuture<'a, ()> {
        Box::pin(async move {
            let mut connection = self.connection.clone();
            if connection.del::<_, ()>(key).await.is_err() {
                self.report_failure(context, "cache/redis/remove", "cache delete failed");
            }
        })
    }
}
