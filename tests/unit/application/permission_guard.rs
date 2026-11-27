use std::{collections::HashMap, sync::Arc};

use pas_hq_app::{
    application::misc::RolePermissionGuard,
    domain::{
        contracts::{
            repository::{
                Pagination, RepositoryFuture,
                role_permission::{GrantArgs, RolePermissionRepository},
            },
            utility::DbExecutor,
        },
        models::{AccessClaims, AppContext, RepositoryError, UsecaseError, entity::RolePermission},
        usecases::shared::PermissionGuard,
    },
    infrastructure::utility::logger::WriterLogger,
};

struct FakeRolePermissions {
    grants: HashMap<String, Vec<String>>,
}

impl RolePermissionRepository for FakeRolePermissions {
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
        Box::pin(async { Ok(None) })
    }

    fn find_by_role_slug<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        role_slug: &'a str,
    ) -> RepositoryFuture<'a, Vec<String>> {
        let permissions = self.grants.get(role_slug).cloned().unwrap_or_default();
        Box::pin(async move { Ok(permissions) })
    }

    fn list_by_role<'a>(
        &'a self,
        _context: &'a AppContext,
        _executor: Option<&'a mut dyn DbExecutor>,
        _role_id: i64,
        _pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<RolePermission>> {
        Box::pin(async { Ok(Vec::new()) })
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

fn guard(grants: HashMap<String, Vec<String>>) -> RolePermissionGuard {
    let logger: Arc<dyn pas_hq_app::domain::contracts::utility::Logger> =
        Arc::new(WriterLogger::new(
            std::io::sink(),
            pas_hq_app::domain::models::logger::Format::Json,
            pas_hq_app::domain::models::logger::Level::None,
        ));

    RolePermissionGuard::new(Arc::new(FakeRolePermissions { grants }), logger)
}

fn context(role: &str) -> AppContext {
    AppContext {
        actor: Some("Ada".into()),
        claims: Some(AccessClaims {
            user_id: 1,
            name: "Ada".into(),
            role_id: 2,
            role_name: "Role".into(),
            role: role.into(),
        }),
        ..AppContext::default()
    }
}

#[tokio::test]
async fn passes_when_every_permission_is_granted() {
    let grants = HashMap::from([(
        "admin".to_owned(),
        vec!["role:create".to_owned(), "role:list".to_owned()],
    )]);
    let guard = guard(grants);
    let context = context("admin");

    let result = guard.require(&context, &["role:create", "role:list"]).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn rejects_when_any_permission_is_missing() {
    let grants = HashMap::from([("cashier".to_owned(), vec!["role:list".to_owned()])]);
    let guard = guard(grants);
    let context = context("cashier");

    let result = guard.require(&context, &["role:list", "role:delete"]).await;

    assert!(matches!(result, Err(UsecaseError::Forbidden)));
}

#[tokio::test]
async fn rejects_when_the_context_has_no_claims() {
    let guard = guard(HashMap::new());
    let anonymous = AppContext::default();

    let result = guard.require(&anonymous, &["role:list"]).await;

    assert!(matches!(result, Err(UsecaseError::Forbidden)));
}

#[tokio::test]
async fn rejects_an_empty_requirement_list() {
    let guard = guard(HashMap::from([("admin".to_owned(), vec![])]));
    let context = context("admin");

    let result = guard.require(&context, &[]).await;

    assert!(matches!(result, Err(UsecaseError::Validation(_))));
}

#[test]
fn guard_is_object_safe() {
    fn accept(_: &dyn PermissionGuard) {}

    let _ = accept;
}
