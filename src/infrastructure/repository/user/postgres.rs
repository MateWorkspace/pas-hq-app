use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                user::{CreateArgs, ListArgs, UpdateArgs, UserRepository},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::User},
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

pub struct PostgresUser {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresUser {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl UserRepository for PostgresUser {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, User> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<User, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::user::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/user/create",
                result,
                || RepositoryError::UserNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<User>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<User>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::user::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/user/find_by_id",
                result,
                || RepositoryError::UserNotFound,
            )
        })
    }

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<User>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<User>, OperationError> = async {
                let mut query = query::list(&filter);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::user::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/user/list",
                result,
                || RepositoryError::UserNotFound,
            )
        })
    }

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, User> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<User, OperationError> = async {
                let mut query = query::update(id, &changes);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::user::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/user/update",
                result,
                || RepositoryError::UserNotFound,
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
                    || RepositoryError::UserNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/user/delete",
                result,
                || RepositoryError::UserNotFound,
            )
        })
    }
}
