use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                Pagination, RepositoryFuture,
                stock_movement::{RecordArgs, StockMovementRepository},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::StockMovement},
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

pub struct PostgresStockMovement {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresStockMovement {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl StockMovementRepository for PostgresStockMovement {
    fn record<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: RecordArgs,
    ) -> RepositoryFuture<'a, StockMovement> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<StockMovement, OperationError> = async {
                let mut query = query::record(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::stock_movement::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/stock_movement/record",
                result,
                || RepositoryError::StockMovementNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<StockMovement>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<StockMovement>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::stock_movement::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/stock_movement/find_by_id",
                result,
                || RepositoryError::StockMovementNotFound,
            )
        })
    }

    fn list_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<StockMovement>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<StockMovement>, OperationError> = async {
                let mut query = query::list_by_sku(sku_id, pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::stock_movement::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/stock_movement/list_by_sku",
                result,
                || RepositoryError::StockMovementNotFound,
            )
        })
    }

    fn list_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        location_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<StockMovement>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<StockMovement>, OperationError> = async {
                let mut query = query::list_by_location(location_id, pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::stock_movement::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/stock_movement/list_by_location",
                result,
                || RepositoryError::StockMovementNotFound,
            )
        })
    }
}
