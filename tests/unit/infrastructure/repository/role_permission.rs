use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use pas_hq_app::{
    domain::{
        contracts::{
            caching::{Cache, CacheFuture, CacheKind},
            repository::{
                Pagination, RepositoryFuture,
                role_permission::{GrantArgs, RolePermissionRepository},
            },
            utility::DbExecutor,
        },
        models::{AppContext, RepositoryError, entity::RolePermission},
    },
    infrastructure::{
        repository::role_permission::CachedRolePermission, utility::transactor::PoolExecutor,
    },
};

#[derive(Default)]
struct FakeCache {
    entries: Mutex<HashMap<String, Vec<u8>>>,
}

impl Cache for FakeCache {
    fn get<'a>(
        &'a self,
        _context: &'a AppContext,
        key: &'a str,
    ) -> CacheFuture<'a, Option<Vec<u8>>> {
        let value = self.entries.lock().unwrap().get(key).cloned();
        Box::pin(async move { value })
    }

    fn set<'a>(
        &'a self,
        _context: &'a AppContext,
        key: &'a str,
        _kind: CacheKind,
        value: Vec<u8>,
    ) -> CacheFuture<'a, ()> {
        self.entries.lock().unwrap().insert(key.to_owned(), value);
        Box::pin(async {})
    }

    fn remove<'a>(&'a self, _context: &'a AppContext, key: &'a str) -> CacheFuture<'a, ()> {
        self.entries.lock().unwrap().remove(key);
        Box::pin(async {})
    }
}

struct FakeRepository {
    permissions: Vec<String>,
    calls: AtomicUsize,
}

impl FakeRepository {
    fn new(permissions: Vec<String>) -> Self {
        Self {
            permissions,
            calls: AtomicUsize::new(0),
        }
    }

    fn calls(&self) -> usize {
        self.calls.load(Ordering::Acquire)
    }
}

impl RolePermissionRepository for FakeRepository {
    fn grant<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        _input: GrantArgs,
    ) -> RepositoryFuture<'a, RolePermission> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }

    fn find<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        _role_id: i64,
        _permission_id: i64,
    ) -> RepositoryFuture<'a, Option<RolePermission>> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }

    fn find_by_role_slug<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        _role_slug: &'a str,
    ) -> RepositoryFuture<'a, Vec<String>> {
        self.calls.fetch_add(1, Ordering::AcqRel);
        let permissions = self.permissions.clone();
        Box::pin(async move { Ok(permissions) })
    }

    fn list_by_role<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        _role_id: i64,
        _pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<RolePermission>> {
        Box::pin(async { Err(RepositoryError::Unimplemented) })
    }

    fn revoke<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        _role_id: i64,
        _permission_id: i64,
    ) -> RepositoryFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }
}

fn decorator() -> (Arc<FakeRepository>, CachedRolePermission) {
    let repository = Arc::new(FakeRepository::new(vec![
        "user.get".to_owned(),
        "user.list".to_owned(),
    ]));
    let cache = Arc::new(FakeCache::default());
    let decorator = CachedRolePermission::new(
        Arc::clone(&repository) as Arc<dyn RolePermissionRepository>,
        cache as Arc<dyn Cache>,
    );

    (repository, decorator)
}

#[tokio::test]
async fn a_miss_populates_the_cache_and_the_next_call_is_a_hit() {
    let (repository, decorator) = decorator();
    let context = AppContext::default();

    let first = decorator
        .find_by_role_slug(&context, None, "admin")
        .await
        .unwrap();
    assert_eq!(first, vec!["user.get".to_owned(), "user.list".to_owned()]);
    assert_eq!(repository.calls(), 1);

    let second = decorator
        .find_by_role_slug(&context, None, "admin")
        .await
        .unwrap();
    assert_eq!(second, first);
    assert_eq!(repository.calls(), 1);
}

#[tokio::test]
async fn revoking_invalidates_cached_permissions() {
    let (repository, decorator) = decorator();
    let context = AppContext::default();

    decorator
        .find_by_role_slug(&context, None, "admin")
        .await
        .unwrap();
    assert_eq!(repository.calls(), 1);

    decorator.revoke(&context, None, 1, 2).await.unwrap();

    decorator
        .find_by_role_slug(&context, None, "admin")
        .await
        .unwrap();
    assert_eq!(repository.calls(), 2);
}

#[tokio::test]
async fn a_transactional_executor_bypasses_the_cache() {
    let (repository, decorator) = decorator();
    let context = AppContext::default();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://localhost/pas_hq")
        .unwrap();
    let mut executor = PoolExecutor::new(pool);

    {
        let executor_ref: &mut dyn DbExecutor = &mut executor;
        decorator
            .find_by_role_slug(&context, Some(executor_ref), "admin")
            .await
            .unwrap();
    }
    {
        let executor_ref: &mut dyn DbExecutor = &mut executor;
        decorator
            .find_by_role_slug(&context, Some(executor_ref), "admin")
            .await
            .unwrap();
    }

    assert_eq!(repository.calls(), 2);
}
