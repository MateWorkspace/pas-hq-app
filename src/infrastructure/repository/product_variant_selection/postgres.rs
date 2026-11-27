use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        contracts::{
            repository::{
                RepositoryFuture,
                product_variant_selection::{
                    CreateArgs, ProductVariantSelectionRepository, UpdateArgs,
                },
            },
            utility::{DbExecutor, Logger},
        },
        models::{AppContext, RepositoryError, entity::ProductVariantSelection},
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

pub struct PostgresProductVariantSelection {
    database: PgPool,
    logger: Arc<dyn Logger>,
}

impl PostgresProductVariantSelection {
    pub fn new(database: PgPool, logger: Arc<dyn Logger>) -> Self {
        Self { database, logger }
    }
}

impl ProductVariantSelectionRepository for PostgresProductVariantSelection {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, ProductVariantSelection> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ProductVariantSelection, OperationError> = async {
                let mut query = query::create(&input);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::product_variant_selection::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant_selection/create",
                result,
                || RepositoryError::ProductVariantSelectionNotFound,
            )
        })
    }

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<ProductVariantSelection>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Option<ProductVariantSelection>, OperationError> = async {
                let mut query = query::find_by_id(id);
                match executor.fetch_optional(query.build()).await? {
                    Some(row) => Ok(Some(scan::product_variant_selection::decode(&row)?)),
                    None => Ok(None),
                }
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant_selection/find_by_id",
                result,
                || RepositoryError::ProductVariantSelectionNotFound,
            )
        })
    }

    fn list_by_variant<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        product_variant_id: i64,
    ) -> RepositoryFuture<'a, Vec<ProductVariantSelection>> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<Vec<ProductVariantSelection>, OperationError> = async {
                let mut query = query::list_by_variant(product_variant_id);
                let rows = executor.fetch_all(query.build()).await?;
                let mut items = Vec::with_capacity(rows.len());
                for row in &rows {
                    items.push(scan::product_variant_selection::decode(row)?);
                }
                Ok(items)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant_selection/list_by_variant",
                result,
                || RepositoryError::ProductVariantSelectionNotFound,
            )
        })
    }

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, ProductVariantSelection> {
        Box::pin(async move {
            let mut pool_executor = PoolExecutor::new(self.database.clone());
            let executor = match executor {
                Some(executor) => &mut *executor,
                None => &mut pool_executor,
            };
            let result: Result<ProductVariantSelection, OperationError> = async {
                let mut query = query::update(id, &changes);
                let row = executor.fetch_one(query.build()).await?;
                Ok(scan::product_variant_selection::decode(&row)?)
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant_selection/update",
                result,
                || RepositoryError::ProductVariantSelectionNotFound,
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
                    || RepositoryError::ProductVariantSelectionNotFound,
                )
            }
            .await;
            finish(
                self.logger.as_ref(),
                context,
                "repository/product_variant_selection/delete",
                result,
                || RepositoryError::ProductVariantSelectionNotFound,
            )
        })
    }
}
