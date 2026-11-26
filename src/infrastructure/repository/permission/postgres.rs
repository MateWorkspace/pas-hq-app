use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                permission::{CreateArgs, ListArgs, PermissionRepository},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::Permission},
    },
    infrastructure::utility::transactor::PoolExecutor,
};

use super::{
    super::shared::{
        error::{OperationError, finish},
        scan,
    },
    postgres_query as query,
};

pub struct PostgresPermission {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresPermission {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl PermissionRepository for PostgresPermission {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, Permission> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Permission, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::permission::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/permission/create",
                result,
                || RepositoryError::PermissionNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<Permission>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Permission>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::permission::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/permission/find_by_id",
                result,
                || RepositoryError::PermissionNotFound,
            )
        })
    }

    fn find_by_slug<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        slug: &'a str,
    ) -> RepositoryFuture<'a, Option<Permission>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Permission>, OperationError> = async {
                let mut query = query::find_by_slug(slug);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::permission::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/permission/find_by_slug",
                result,
                || RepositoryError::PermissionNotFound,
            )
        })
    }

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<Permission>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<Permission>, OperationError> = async {
                let mut query = query::list(&filter);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::permission::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/permission/list",
                result,
                || RepositoryError::PermissionNotFound,
            )
        })
    }
}
