use std::sync::Arc;

use pas_hq_app::{
    application::auth::ApiKeyAuthentication,
    domain::{
        contracts::{
            caching::{ApiKeyClaimsCache, Cache, CacheFuture, CacheKind},
            repository::{
                Pagination, RepositoryFuture,
                api_key::{ApiKeyAuth as ApiKeyAuthRow, ApiKeyRepository, CreateArgs, RotateArgs},
            },
            utility::{ApiKey as ApiKeyGenerator, DbExecutor},
        },
        models::{AppContext, RepositoryError, UsecaseError, entity::ApiKey as ApiKeyEntity},
        usecases::auth::ApiKeyAuth,
    },
    infrastructure::caching::CachedApiKeyClaims,
};

#[derive(Default)]
struct FakeCache {
    entries: std::sync::Mutex<std::collections::HashMap<String, Vec<u8>>>,
}

impl Cache for FakeCache {
    fn get<'a>(&'a self, _: &'a AppContext, key: &'a str) -> CacheFuture<'a, Option<Vec<u8>>> {
        let value = self.entries.lock().unwrap().get(key).cloned();
        Box::pin(async move { value })
    }

    fn set<'a>(
        &'a self,
        _: &'a AppContext,
        key: &'a str,
        _: CacheKind,
        value: Vec<u8>,
    ) -> CacheFuture<'a, ()> {
        self.entries.lock().unwrap().insert(key.to_owned(), value);
        Box::pin(async {})
    }

    fn remove<'a>(&'a self, _: &'a AppContext, key: &'a str) -> CacheFuture<'a, ()> {
        self.entries.lock().unwrap().remove(key);
        Box::pin(async {})
    }
}

struct FakeKeys {
    auth: Option<ApiKeyAuthRow>,
    calls: std::sync::atomic::AtomicUsize,
}

impl FakeKeys {
    fn new(auth: Option<ApiKeyAuthRow>) -> Self {
        Self {
            auth,
            calls: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn calls(&self) -> usize {
        self.calls.load(std::sync::atomic::Ordering::Acquire)
    }
}

impl ApiKeyRepository for FakeKeys {
    fn create<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: CreateArgs,
    ) -> RepositoryFuture<'a, ApiKeyEntity> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }
    fn find_by_id<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: i64,
    ) -> RepositoryFuture<'a, Option<ApiKeyEntity>> {
        Box::pin(async { Ok(None) })
    }
    fn list_by_user<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: i64,
        _: Pagination,
    ) -> RepositoryFuture<'a, Vec<ApiKeyEntity>> {
        Box::pin(async { Ok(Vec::new()) })
    }
    fn list_all<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: Pagination,
    ) -> RepositoryFuture<'a, Vec<ApiKeyEntity>> {
        Box::pin(async { Ok(Vec::new()) })
    }
    fn rotate<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: i64,
        _: RotateArgs,
    ) -> RepositoryFuture<'a, ApiKeyEntity> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }
    fn set_active<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: i64,
        _: bool,
        _: Option<i64>,
    ) -> RepositoryFuture<'a, ApiKeyEntity> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }
    fn delete<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: i64,
    ) -> RepositoryFuture<'a, ApiKeyEntity> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }
    fn delete_by_user_id<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: i64,
    ) -> RepositoryFuture<'a, Vec<String>> {
        Box::pin(async { Ok(Vec::new()) })
    }
    fn find_auth_by_hash<'a>(
        &'a self,
        _: &'a AppContext,
        _: Option<&'a mut dyn DbExecutor>,
        _: &'a str,
    ) -> RepositoryFuture<'a, Option<ApiKeyAuthRow>> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let auth = self.auth.clone();
        Box::pin(async move { Ok(auth) })
    }
}

fn logger() -> Arc<dyn pas_hq_app::domain::contracts::utility::Logger> {
    Arc::new(
        pas_hq_app::infrastructure::utility::logger::WriterLogger::new(
            std::io::sink(),
            pas_hq_app::domain::models::logger::Format::Json,
            pas_hq_app::domain::models::logger::Level::None,
        ),
    )
}

fn auth_row(key_active: bool, user_active: bool) -> ApiKeyAuthRow {
    ApiKeyAuthRow {
        api_key_id: 1,
        key_is_active: key_active,
        user_id: 7,
        user_name: "Ada".into(),
        user_is_active: user_active,
        suspended_until: None,
        role_id: 2,
        role_name: "Administrator".into(),
        role_slug: "admin".into(),
    }
}

fn build(auth: Option<ApiKeyAuthRow>) -> (Arc<FakeKeys>, ApiKeyAuthentication, Arc<FakeCache>) {
    let keys = Arc::new(FakeKeys::new(auth));
    let backend = Arc::new(FakeCache::default());
    let cache: Arc<dyn ApiKeyClaimsCache> = Arc::new(CachedApiKeyClaims::new(
        Arc::clone(&backend) as Arc<dyn Cache>,
        logger(),
        std::time::Duration::from_secs(60),
    ));
    let generator: Arc<dyn ApiKeyGenerator> =
        Arc::new(pas_hq_app::infrastructure::utility::api_key::Sha256ApiKey);

    let auth = ApiKeyAuthentication::new(
        Arc::clone(&keys) as Arc<dyn ApiKeyRepository>,
        cache,
        generator,
        logger(),
    );

    (keys, auth, backend)
}

fn valid_key() -> String {
    pas_hq_app::domain::contracts::utility::ApiKey::generate(
        &pas_hq_app::infrastructure::utility::api_key::Sha256ApiKey,
    )
    .unwrap()
    .raw
}

#[tokio::test]
async fn a_valid_key_resolves_claims_and_is_cached() {
    let (keys, auth, _) = build(Some(auth_row(true, true)));
    let context = AppContext::default();
    let raw = valid_key();

    let claims = auth.validate_api_key(&context, &raw).await.unwrap();
    assert_eq!(claims.user_id, 7);
    assert_eq!(claims.role, "admin");
    assert_eq!(keys.calls(), 1);

    let cached = auth.validate_api_key(&context, &raw).await.unwrap();
    assert_eq!(cached, claims);
    assert_eq!(
        keys.calls(),
        1,
        "second validation should be served from cache"
    );
}

#[tokio::test]
async fn a_disabled_key_is_rejected() {
    let (_, auth, _) = build(Some(auth_row(false, true)));

    let result = auth
        .validate_api_key(&AppContext::default(), &valid_key())
        .await;

    assert!(matches!(result, Err(UsecaseError::Unauthorized)));
}

#[tokio::test]
async fn an_inactive_owner_is_rejected() {
    let (_, auth, _) = build(Some(auth_row(true, false)));

    let result = auth
        .validate_api_key(&AppContext::default(), &valid_key())
        .await;

    assert!(matches!(result, Err(UsecaseError::Unauthorized)));
}

#[tokio::test]
async fn an_unknown_key_is_rejected() {
    let (_, auth, _) = build(None);

    let result = auth
        .validate_api_key(&AppContext::default(), &valid_key())
        .await;

    assert!(matches!(result, Err(UsecaseError::Unauthorized)));
}

#[tokio::test]
async fn a_malformed_key_is_rejected_without_touching_the_repository() {
    let (keys, auth, _) = build(Some(auth_row(true, true)));

    let result = auth
        .validate_api_key(&AppContext::default(), "not-a-key")
        .await;

    assert!(matches!(result, Err(UsecaseError::Unauthorized)));
    assert_eq!(keys.calls(), 0);
}

#[tokio::test]
async fn removing_a_cached_entry_forces_a_repository_read() {
    let (keys, auth, backend) = build(Some(auth_row(true, true)));
    let context = AppContext::default();
    let raw = valid_key();

    auth.validate_api_key(&context, &raw).await.unwrap();
    assert_eq!(keys.calls(), 1);

    let hash = pas_hq_app::domain::contracts::utility::ApiKey::hash(
        &pas_hq_app::infrastructure::utility::api_key::Sha256ApiKey,
        &raw,
    )
    .unwrap();
    let cache = CachedApiKeyClaims::new(
        Arc::clone(&backend) as Arc<dyn Cache>,
        logger(),
        std::time::Duration::from_secs(60),
    );
    cache.remove(&context, &hash).await;

    auth.validate_api_key(&context, &raw).await.unwrap();
    assert_eq!(keys.calls(), 2, "invalidated key must be re-read");
}
