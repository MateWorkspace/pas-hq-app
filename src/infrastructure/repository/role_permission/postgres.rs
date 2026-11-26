use std::sync::Arc;

use sqlx::{PgPool, Row};

use crate::{
    domain::{
        contracts::{
            repository::{
                Pagination, RepositoryFuture,
                role_permission::{GrantArgs, RolePermissionRepository},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::RolePermission},
    },
    infrastructure::utility::transactor::PoolExecutor,
};

use super::{
    super::shared::{
        error::{OperationError, finish, require_affected},
        scan,
    },
    postgres_query as query,
};

pub struct PostgresRolePermission {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresRolePermission {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl RolePermissionRepository for PostgresRolePermission {
    fn grant<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: GrantArgs,
    ) -> RepositoryFuture<'a, RolePermission> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<RolePermission, OperationError> = async {
                let mut query = query::grant(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::role_permission::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role_permission/grant",
                result,
                || RepositoryError::RolePermissionNotFound,
            )
        })
    }

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        permission_id: i64,
    ) -> RepositoryFuture<'a, Option<RolePermission>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<RolePermission>, OperationError> = async {
                let mut query = query::find(role_id, permission_id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::role_permission::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role_permission/find",
                result,
                || RepositoryError::RolePermissionNotFound,
            )
        })
    }

    fn find_by_role_slug<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_slug: &'a str,
    ) -> RepositoryFuture<'a, Vec<String>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<String>, OperationError> = async {
                let mut query = query::find_by_role_slug(role_slug);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(row.try_get("slug")?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role_permission/find_by_role_slug",
                result,
                || RepositoryError::RolePermissionNotFound,
            )
        })
    }

    fn list_by_role<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<RolePermission>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<RolePermission>, OperationError> = async {
                let mut query = query::list_by_role(role_id, pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::role_permission::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role_permission/list_by_role",
                result,
                || RepositoryError::RolePermissionNotFound,
            )
        })
    }

    fn revoke<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        role_id: i64,
        permission_id: i64,
    ) -> RepositoryFuture<'a, ()> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<(), OperationError> = async {
                let mut query = query::revoke(role_id, permission_id);
                require_affected(
                    executor.execute(query.build()).await?.rows_affected(),
                    || RepositoryError::RolePermissionNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role_permission/revoke",
                result,
                || RepositoryError::RolePermissionNotFound,
            )
        })
    }
}
