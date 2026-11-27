use std::sync::Arc;

use sqlx::{PgPool, Row};

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                product_sku::{CreateArgs, ProductSkuRepository},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::ProductSku},
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

pub struct PostgresProductSku {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresProductSku {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl ProductSkuRepository for PostgresProductSku {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ProductSku> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ProductSku, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::product_sku::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_sku/create",
                result,
                || RepositoryError::ProductSkuNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ProductSku>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<ProductSku>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::product_sku::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_sku/find_by_id",
                result,
                || RepositoryError::ProductSkuNotFound,
            )
        })
    }

    fn find_by_selection_key<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_id: i64,
        selection_key: &'a str,
    ) -> RepositoryFuture<'a, Option<ProductSku>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<ProductSku>, OperationError> = async {
                let mut query = query::find_by_selection_key(product_id, selection_key);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::product_sku::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_sku/find_by_selection_key",
                result,
                || RepositoryError::ProductSkuNotFound,
            )
        })
    }

    fn list_by_product<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_id: i64,
    ) -> RepositoryFuture<'a, Vec<ProductSku>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<ProductSku>, OperationError> = async {
                let mut query = query::list_by_product(product_id);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::product_sku::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_sku/list_by_product",
                result,
                || RepositoryError::ProductSkuNotFound,
            )
        })
    }

    fn list_selection_ids<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        sku_id: i64,
    ) -> RepositoryFuture<'a, Vec<i64>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<i64>, OperationError> = async {
                let mut query = query::list_selection_ids(sku_id);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(row.try_get("variant_selection_id")?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_sku/list_selection_ids",
                result,
                || RepositoryError::ProductSkuNotFound,
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
                    || RepositoryError::ProductSkuNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_sku/delete",
                result,
                || RepositoryError::ProductSkuNotFound,
            )
        })
    }
}
