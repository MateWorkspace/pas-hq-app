use std::sync::Arc;

use crate::{
    application::shared::permissions::{PRICE_LIST, PRICE_REMOVE, PRICE_SET},
    domain::{
        contracts::{
            repository::price::{PriceRepository, SetArgs},
            utility::Logger,
        },
        models::{AppContext, RepositoryError, UsecaseError, entity::Price, logger::Meta},
        usecases::{
            UsecaseFuture,
            catalog::price::{EffectivePriceRequest, ManagePrice, PriceKey, SetPrice},
            shared::PermissionGuard,
        },
    },
};

pub struct PriceManagement {
    prices: Arc<dyn PriceRepository>,
    permission_guard: Arc<dyn PermissionGuard>,
    logger: Arc<dyn Logger>,
}

impl PriceManagement {
    pub fn new(
        prices: Arc<dyn PriceRepository>,
        permission_guard: Arc<dyn PermissionGuard>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            prices,
            permission_guard,
            logger,
        }
    }

    fn log_repository_error(&self, context: &AppContext, tag: &str, error: &RepositoryError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }
}

impl ManagePrice for PriceManagement {
    fn set_price<'a>(
        &'a self,
        context: &'a AppContext,
        input: SetPrice,
    ) -> UsecaseFuture<'a, Price> {
        Box::pin(async move {
            self.permission_guard.require(context, &[PRICE_SET]).await?;

            match self
                .prices
                .set(
                    context,
                    None,
                    SetArgs {
                        sku_id: input.sku_id,
                        location_id: input.location_id,
                        currency: input.currency,
                        amount: input.amount,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(price) => {
                    let meta = Meta::from([
                        ("price_id".into(), price.id.into()),
                        ("sku_id".into(), price.sku_id.into()),
                    ]);
                    self.logger.info(
                        context,
                        "application/catalog/price/set",
                        "price set",
                        Some(&meta),
                    );
                    Ok(price)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/catalog/price/set", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn remove_price<'a>(
        &'a self,
        context: &'a AppContext,
        input: PriceKey,
    ) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRICE_REMOVE])
                .await?;

            match self
                .prices
                .remove(
                    context,
                    None,
                    input.sku_id,
                    input.location_id,
                    &input.currency,
                )
                .await
            {
                Ok(()) => {
                    let meta = Meta::from([
                        ("sku_id".into(), input.sku_id.into()),
                        ("has_location".into(), input.location_id.is_some().into()),
                    ]);
                    self.logger.info(
                        context,
                        "application/catalog/price/remove",
                        "price removed",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(context, "application/catalog/price/remove", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_prices<'a>(
        &'a self,
        context: &'a AppContext,
        sku_id: i64,
    ) -> UsecaseFuture<'a, Vec<Price>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRICE_LIST])
                .await?;

            self.prices
                .list_by_sku(context, None, sku_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/catalog/price/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn get_effective_price<'a>(
        &'a self,
        context: &'a AppContext,
        input: EffectivePriceRequest,
    ) -> UsecaseFuture<'a, Option<Price>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRICE_LIST])
                .await?;

            self.prices
                .find_effective(
                    context,
                    None,
                    input.sku_id,
                    input.location_id,
                    &input.currency,
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/catalog/price/get_effective",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }
}
