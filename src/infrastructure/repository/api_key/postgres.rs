use std::sync::Arc;

use sqlx::{PgPool, Row};

use crate::{
    domain::{
        contracts::{
            repository::{
                Pagination, RepositoryFuture,
                api_key::{ApiKeyAuth, ApiKeyRepository, CreateArgs, RotateArgs},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::ApiKey},
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

pub struct PostgresApiKey {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresApiKey {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl ApiKeyRepository for PostgresApiKey {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ApiKey> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ApiKey, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::api_key::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/create",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ApiKey>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<ApiKey>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::api_key::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/find_by_id",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn list_by_user<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        user_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<ApiKey>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<ApiKey>, OperationError> = async {
                let mut query = query::list_by_user(user_id, pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::api_key::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/list_by_user",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn list_all<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<ApiKey>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<ApiKey>, OperationError> = async {
                let mut query = query::list_all(pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::api_key::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/list_all",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn rotate<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        input: RotateArgs,
    ) -> RepositoryFuture<'a, ApiKey> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ApiKey, OperationError> = async {
                let mut query = query::rotate(id, &input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::api_key::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/rotate",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn set_active<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        is_active: bool,
        updated_by: Option<i64>,
    ) -> RepositoryFuture<'a, ApiKey> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ApiKey, OperationError> = async {
                let mut query = query::set_active(id, is_active, updated_by);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::api_key::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/set_active",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ApiKey> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ApiKey, OperationError> = async {
                let mut query = query::delete(id);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::api_key::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/delete",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn delete_by_user_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        user_id: i64,
    ) -> RepositoryFuture<'a, Vec<String>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<String>, OperationError> = async {
                let mut query = query::delete_by_user_id(user_id);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(row.try_get("hashed_key")?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/delete_by_user_id",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }

    fn find_auth_by_hash<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        hashed_key: &'a str,
    ) -> RepositoryFuture<'a, Option<ApiKeyAuth>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<ApiKeyAuth>, OperationError> = async {
                let mut query = query::find_auth_by_hash(hashed_key);
                let Some(row) = executor.fetch_optional(query.build()).await? else {
                    return Ok(None);
                };

                Ok(Some(ApiKeyAuth {
                    api_key_id: row.try_get("api_key_id")?,
                    key_is_active: row.try_get("key_is_active")?,
                    user_id: row.try_get("user_id")?,
                    user_name: row.try_get("user_name")?,
                    user_is_active: row.try_get("user_is_active")?,
                    suspended_until: row.try_get("suspended_until")?,
                    role_id: row.try_get("role_id")?,
                    role_name: row.try_get("role_name")?,
                    role_slug: row.try_get("role_slug")?,
                }))
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/api_key/find_auth_by_hash",
                result,
                || RepositoryError::ApiKeyNotFound,
            )
        })
    }
}
