use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                price::{PriceRepository, SetArgs},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::Price},
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

pub struct PostgresPrice {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresPrice {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl PriceRepository for PostgresPrice {
    fn set<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: SetArgs,
    ) -> RepositoryFuture<'a, Price> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Price, OperationError> = async {
                let mut query = query::set(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::price::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/price/set",
                result,
                || RepositoryError::PriceNotFound,
            )
        })
    }

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        location_id: Option<i64>,
        currency: &'a str,
    ) -> RepositoryFuture<'a, Option<Price>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Price>, OperationError> = async {
                let mut query = query::find(sku_id, location_id, currency);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::price::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/price/find",
                result,
                || RepositoryError::PriceNotFound,
            )
        })
    }

    fn find_effective<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        location_id: i64,
        currency: &'a str,
    ) -> RepositoryFuture<'a, Option<Price>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Price>, OperationError> = async {
                let mut query = query::find_effective(sku_id, location_id, currency);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::price::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/price/find_effective",
                result,
                || RepositoryError::PriceNotFound,
            )
        })
    }

    fn list_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
    ) -> RepositoryFuture<'a, Vec<Price>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<Price>, OperationError> = async {
                let mut query = query::list_by_sku(sku_id);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::price::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/price/list_by_sku",
                result,
                || RepositoryError::PriceNotFound,
            )
        })
    }

    fn remove<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        location_id: Option<i64>,
        currency: &'a str,
    ) -> RepositoryFuture<'a, ()> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<(), OperationError> = async {
                let mut query = query::remove(sku_id, location_id, currency);
                require_affected(
                    executor.execute(query.build()).await?.rows_affected(),
                    || RepositoryError::PriceNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/price/remove",
                result,
                || RepositoryError::PriceNotFound,
            )
        })
    }
}
