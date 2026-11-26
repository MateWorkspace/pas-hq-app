use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                role::{CreateArgs, ListArgs, RoleRepository, UpdateArgs},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::Role},
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

pub struct PostgresRole {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresRole {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl RoleRepository for PostgresRole {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, Role> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Role, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::role::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role/create",
                result,
                || RepositoryError::RoleNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<Role>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Role>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::role::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role/find_by_id",
                result,
                || RepositoryError::RoleNotFound,
            )
        })
    }

    fn find_by_slug<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        slug: &'a str,
    ) -> RepositoryFuture<'a, Option<Role>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Role>, OperationError> = async {
                let mut query = query::find_by_slug(slug);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::role::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role/find_by_slug",
                result,
                || RepositoryError::RoleNotFound,
            )
        })
    }

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<Role>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<Role>, OperationError> = async {
                let mut query = query::list(&filter);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::role::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role/list",
                result,
                || RepositoryError::RoleNotFound,
            )
        })
    }

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, Role> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Role, OperationError> = async {
                let mut query = query::update(id, &changes);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::role::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role/update",
                result,
                || RepositoryError::RoleNotFound,
            )
        })
    }

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<(), OperationError> = async {
                let mut query = query::delete(id);
                require_affected(
                    executor.execute(query.build()).await?.rows_affected(),
                    || RepositoryError::RoleNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/role/delete",
                result,
                || RepositoryError::RoleNotFound,
            )
        })
    }
}
