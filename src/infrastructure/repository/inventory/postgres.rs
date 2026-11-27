use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                Pagination, RepositoryFuture,
                inventory::{InventoryRepository, SetStockArgs},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::Inventory},
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

pub struct PostgresInventory {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresInventory {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl InventoryRepository for PostgresInventory {
    fn set_stock<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: SetStockArgs,
    ) -> RepositoryFuture<'a, Inventory> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Inventory, OperationError> = async {
                let mut query = query::set_stock(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::inventory::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/inventory/set_stock",
                result,
                || RepositoryError::InventoryNotFound,
            )
        })
    }

    fn find<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        location_id: i64,
        sku_id: i64,
    ) -> RepositoryFuture<'a, Option<Inventory>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<Inventory>, OperationError> = async {
                let mut query = query::find(location_id, sku_id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::inventory::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/inventory/find",
                result,
                || RepositoryError::InventoryNotFound,
            )
        })
    }

    fn list_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        location_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<Inventory>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<Inventory>, OperationError> = async {
                let mut query = query::list_by_location(location_id, pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::inventory::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/inventory/list_by_location",
                result,
                || RepositoryError::InventoryNotFound,
            )
        })
    }

    fn list_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
        pagination: Pagination,
    ) -> RepositoryFuture<'a, Vec<Inventory>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<Inventory>, OperationError> = async {
                let mut query = query::list_by_sku(sku_id, pagination);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::inventory::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/inventory/list_by_sku",
                result,
                || RepositoryError::InventoryNotFound,
            )
        })
    }
}
