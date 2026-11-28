use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};

use crate::domain::{
    contracts::{
        caching::{ApiKeyClaimsCache, Cache, CacheFuture, CacheKind},
        utility::Logger,
    },
    models::{AccessClaims, AppContext, logger::Meta},
};

/// Cached claims projection.
///
/// The domain claims type stays free of serde; this adapter owns the wire shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedClaims {
    user_id: i64,
    name: String,
    role_id: i64,
    role_name: String,
    role: String,
}

impl From<&AccessClaims> for CachedClaims {
    fn from(value: &AccessClaims) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name.clone(),
            role_id: value.role_id,
            role_name: value.role_name.clone(),
            role: value.role.clone(),
        }
    }
}

impl From<CachedClaims> for AccessClaims {
    fn from(value: CachedClaims) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            role_id: value.role_id,
            role_name: value.role_name,
            role: value.role,
        }
    }
}

/// Wraps any cache backend to store claims per key digest.
///
/// Serialization or backend failures are logged and treated as a miss so
/// validation always falls back to the database rather than failing.
pub struct CachedApiKeyClaims {
    cache: Arc<dyn Cache>,
    logger: Arc<dyn Logger>,
    ttl: Duration,
}

impl CachedApiKeyClaims {
    pub fn new(cache: Arc<dyn Cache>, logger: Arc<dyn Logger>, ttl: Duration) -> Self {
        Self { cache, logger, ttl }
    }

    fn key(key_hash: &str) -> String {
        format!("pas_hq:v1:api_key:hash:{key_hash}")
    }

    fn warn(&self, context: &AppContext, tag: &str, message: &str) {
        let meta = Meta::from([("ttl_secs".into(), self.ttl.as_secs().into())]);
        self.logger.warn(context, tag, message, Some(&meta));
    }
}

impl ApiKeyClaimsCache for CachedApiKeyClaims {
    fn get<'a>(
        &'a self,
        context: &'a AppContext,
        key_hash: &'a str,
    ) -> CacheFuture<'a, Option<AccessClaims>> {
        let key = Self::key(key_hash);
        Box::pin(async move {
            let value = self.cache.get(context, &key).await?;
            match serde_json::from_slice::<CachedClaims>(&value) {
                Ok(claims) => Some(claims.into()),
                Err(_) => {
                    self.warn(
                        context,
                        "cache/api_key/get",
                        "cached api key claims could not be decoded and were ignored",
                    );
                    None
                }
            }
        })
    }

    fn set<'a>(
        &'a self,
        context: &'a AppContext,
        key_hash: &'a str,
        claims: &'a AccessClaims,
    ) -> CacheFuture<'a, ()> {
        let key = Self::key(key_hash);
        let encoded = serde_json::to_vec(&CachedClaims::from(claims)).ok();
        Box::pin(async move {
            let Some(value) = encoded else {
                self.warn(
                    context,
                    "cache/api_key/set",
                    "api key claims could not be encoded for caching",
                );
                return;
            };

            self.cache
                .set(context, &key, CacheKind::Individual, value)
                .await;
        })
    }

    fn remove<'a>(&'a self, context: &'a AppContext, key_hash: &'a str) -> CacheFuture<'a, ()> {
        let key = Self::key(key_hash);
        Box::pin(async move {
            self.cache.remove(context, &key).await;
        })
    }
}
