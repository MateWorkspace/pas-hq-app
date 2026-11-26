use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                base_auth::{BaseAuthRepository, CreateArgs, UpdateArgs},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::BaseAuth},
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

pub struct PostgresBaseAuth {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresBaseAuth {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl BaseAuthRepository for PostgresBaseAuth {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, BaseAuth> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<BaseAuth, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::base_auth::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/base_auth/create",
                result,
                || RepositoryError::BaseAuthNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<BaseAuth>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<BaseAuth>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::base_auth::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/base_auth/find_by_id",
                result,
                || RepositoryError::BaseAuthNotFound,
            )
        })
    }

    fn find_by_user_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        user_id: i64,
    ) -> RepositoryFuture<'a, Option<BaseAuth>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<BaseAuth>, OperationError> = async {
                let mut query = query::find_by_user_id(user_id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::base_auth::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/base_auth/find_by_user_id",
                result,
                || RepositoryError::BaseAuthNotFound,
            )
        })
    }

    fn find_by_username<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        username: &'a str,
    ) -> RepositoryFuture<'a, Option<BaseAuth>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<BaseAuth>, OperationError> = async {
                let mut query = query::find_by_username(username);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::base_auth::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/base_auth/find_by_username",
                result,
                || RepositoryError::BaseAuthNotFound,
            )
        })
    }

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, BaseAuth> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<BaseAuth, OperationError> = async {
                let mut query = query::update(id, &changes);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::base_auth::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/base_auth/update",
                result,
                || RepositoryError::BaseAuthNotFound,
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
                    || RepositoryError::BaseAuthNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/base_auth/delete",
                result,
                || RepositoryError::BaseAuthNotFound,
            )
        })
    }
}
