use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                product_variant::{CreateArgs, ProductVariantRepository, UpdateArgs},
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::ProductVariant},
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

pub struct PostgresProductVariant {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresProductVariant {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl ProductVariantRepository for PostgresProductVariant {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ProductVariant> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ProductVariant, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::product_variant::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant/create",
                result,
                || RepositoryError::ProductVariantNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ProductVariant>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<ProductVariant>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::product_variant::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant/find_by_id",
                result,
                || RepositoryError::ProductVariantNotFound,
            )
        })
    }

    fn list_by_product<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_id: i64,
    ) -> RepositoryFuture<'a, Vec<ProductVariant>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<ProductVariant>, OperationError> = async {
                let mut query = query::list_by_product(product_id);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::product_variant::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant/list_by_product",
                result,
                || RepositoryError::ProductVariantNotFound,
            )
        })
    }

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, ProductVariant> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ProductVariant, OperationError> = async {
                let mut query = query::update(id, &changes);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::product_variant::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant/update",
                result,
                || RepositoryError::ProductVariantNotFound,
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
                    || RepositoryError::ProductVariantNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant/delete",
                result,
                || RepositoryError::ProductVariantNotFound,
            )
        })
    }
}
