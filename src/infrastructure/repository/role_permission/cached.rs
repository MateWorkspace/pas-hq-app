use std::sync::Arc;

use crate::domain::{
    contracts::{
        caching::{Cache, CacheKind},
        repository::{
            Pagination, RepositoryFuture,
            role_permission::{GrantArgs, RolePermissionRepository},
        },
        utility::DbExecutor,
    },
    models::{AppContext, entity::RolePermission},
};

use super::super::shared::cache::CacheNamespace;

/// Cache-aside decorator for the role-to-permission lookup.
///
/// `find_by_role_slug` reads through the cache; grants and revocations invalidate
/// the whole namespace. Reads inside a transaction bypass the cache so they never
/// observe uncommitted state.
pub struct CachedRolePermission {
    repository: Arc<dyn RolePermissionRepository>,
    cache: Arc<dyn Cache>,
    namespace: CacheNamespace,
}

impl CachedRolePermission {
    pub fn new(repository: Arc<dyn RolePermissionRepository>, cache: Arc<dyn Cache>) -> Self {
        Self {
            repository,
            cache,
            namespace: CacheNamespace::new("role_permission"),
        }
    }
}

impl RolePermissionRepository for CachedRolePermission {
    fn grant<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: GrantArgs,
    ) -> RepositoryFuture<'a, RolePermission> {
        Box::pin(async move {
            let _guard = self.namespace.invalidation_guard();
            self.repository.grant(context, executor, input).await
        })
    }

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        permission_id: i64,
    ) -> RepositoryFuture<'a, Option<RolePermission>> {
        self.repository
            .find(context, executor, role_id, permission_id)
    }

    fn find_by_role_slug<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_slug: &'a str,
    ) -> RepositoryFuture<'a, Vec<String>> {
        if executor.is_some() {
            return self
                .repository
                .find_by_role_slug(context, executor, role_slug);
        }

        let generation = self.namespace.generation();
        let key = self.namespace.text_key(generation, "slug", role_slug);

        Box::pin(async move {
            if let Some(value) = self.cache.get(context, &key).await
                && let Ok(permissions) = serde_json::from_slice::<Vec<String>>(&value)
            {
                return Ok(permissions);
            }

            let permissions = self
                .repository
                .find_by_role_slug(context, None, role_slug)
                .await?;
            if let Ok(value) = serde_json::to_vec(&permissions)
                && self.namespace.generation() == generation
            {
                self.cache.set(context, &key, CacheKind::List, value).await;
            }

            Ok(permissions)
        })
    }

    fn list_by_role<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<RolePermission>> {
        self.repository
            .list_by_role(context, executor, role_id, pagination)
    }

    fn revoke<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        permission_id: i64,
    ) -> RepositoryFuture<'a, ()> {
        Box::pin(async move {
            let _guard = self.namespace.invalidation_guard();
            self.repository
                .revoke(context, executor, role_id, permission_id)
                .await
        })
    }
}
