use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use crate::{
    application::shared::permissions::{SKU_DELETE, SKU_LIST, SKU_REGENERATE, SKU_UPDATE},
    domain::{
        contracts::{
            repository::{
                Pagination,
                inventory::InventoryRepository,
                product_sku::{CreateArgs, ProductSkuRepository},
                product_variant::ProductVariantRepository,
                product_variant_selection::ProductVariantSelectionRepository,
            },
            utility::Logger,
        },
        models::{AppContext, RepositoryError, UsecaseError, entity::ProductSku, logger::Meta},
        usecases::{
            UsecaseFuture,
            catalog::sku::{
                ManageSku, RegenerateProductSkus, RegenerateProductSkusInput, RemovedSku,
                SetSkuCode, SkuRegenerationPlan,
            },
            shared::PermissionGuard,
        },
    },
};

type Combination = (String, Vec<i64>);

/// Generated SKUs, including the reconcile operation that keeps them valid.
use crate::{
    domain::contracts::utility::Transactor, infrastructure::utility::transactor::PgTransactor,
};

pub struct SkuManagement {
    skus: Arc<dyn ProductSkuRepository>,
    variants: Arc<dyn ProductVariantRepository>,
    selections: Arc<dyn ProductVariantSelectionRepository>,
    inventories: Arc<dyn InventoryRepository>,
    transactor: Arc<PgTransactor>,
    permission_guard: Arc<dyn PermissionGuard>,
    logger: Arc<dyn Logger>,
}

impl SkuManagement {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        skus: Arc<dyn ProductSkuRepository>,
        variants: Arc<dyn ProductVariantRepository>,
        selections: Arc<dyn ProductVariantSelectionRepository>,
        inventories: Arc<dyn InventoryRepository>,
        transactor: Arc<PgTransactor>,
        permission_guard: Arc<dyn PermissionGuard>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            skus,
            variants,
            selections,
            inventories,
            transactor,
            permission_guard,
            logger,
        }
    }

    fn log_usecase_error(&self, context: &AppContext, tag: &str, error: &UsecaseError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    fn log_repository_error(&self, context: &AppContext, tag: &str, error: &RepositoryError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    /// Build the cartesian product of one selection per variant.
    ///
    /// A product with no variants yields one empty combination, which is the key of
    /// a variant-less SKU. A variant with no selections yields no combination at
    /// all, because its design is still incomplete.
    async fn valid_combinations(
        &self,
        context: &AppContext,
        product_id: i64,
        tag: &str,
    ) -> Result<Vec<Combination>, UsecaseError> {
        let variants = self
            .variants
            .list_by_product(context, None, product_id)
            .await
            .map_err(|error| {
                self.log_repository_error(context, tag, &error);
                UsecaseError::Repository(error)
            })?;

        if variants.is_empty() {
            return Ok(vec![(String::new(), Vec::new())]);
        }

        let mut combinations: Vec<Vec<i64>> = vec![Vec::new()];
        for variant in &variants {
            let selections = self
                .selections
                .list_by_variant(context, None, variant.id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, tag, &error);
                    UsecaseError::Repository(error)
                })?;

            if selections.is_empty() {
                self.logger.warn(
                    context,
                    tag,
                    "variant has no selections, so no SKUs can be generated yet",
                    Some(&Meta::from([("variant_id".into(), variant.id.into())])),
                );
                return Ok(Vec::new());
            }

            let mut next = Vec::with_capacity(combinations.len() * selections.len());
            for prefix in &combinations {
                for selection in &selections {
                    let mut extended = prefix.clone();
                    extended.push(selection.id);
                    next.push(extended);
                }
            }
            combinations = next;
        }

        let mut keys = Vec::with_capacity(combinations.len());
        for mut selection_ids in combinations {
            selection_ids.sort_unstable();
            let key = selection_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("-");
            keys.push((key, selection_ids));
        }

        Ok(keys)
    }
}

impl ManageSku for SkuManagement {
    fn list_skus<'a>(
        &'a self,
        context: &'a AppContext,
        product_id: i64,
    ) -> UsecaseFuture<'a, Vec<ProductSku>> {
        Box::pin(async move {
            self.permission_guard.require(context, &[SKU_LIST]).await?;

            self.skus
                .list_by_product(context, None, product_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/catalog/sku/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn set_sku_code<'a>(
        &'a self,
        context: &'a AppContext,
        input: SetSkuCode,
    ) -> UsecaseFuture<'a, ProductSku> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SKU_UPDATE])
                .await?;

            match self
                .skus
                .update_code(context, None, input.sku_id, input.code, context.actor_id())
                .await
            {
                Ok(sku) => {
                    let meta = Meta::from([("sku_id".into(), sku.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/sku/set_code",
                        "SKU code updated",
                        Some(&meta),
                    );
                    Ok(sku)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/catalog/sku/set_code", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_sku<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SKU_DELETE])
                .await?;

            match self.skus.delete(context, None, id).await {
                Ok(()) => {
                    let meta = Meta::from([("sku_id".into(), id.into())]);
                    self.logger.warn(
                        context,
                        "application/catalog/sku/delete",
                        "SKU deleted; its prices and stock were cascaded",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(context, "application/catalog/sku/delete", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}

impl RegenerateProductSkus for SkuManagement {
    fn regenerate_product_skus<'a>(
        &'a self,
        context: &'a AppContext,
        input: RegenerateProductSkusInput,
    ) -> UsecaseFuture<'a, SkuRegenerationPlan> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SKU_REGENERATE])
                .await?;

            let tag = "application/catalog/sku/regenerate";

            let combinations = self
                .valid_combinations(context, input.product_id, tag)
                .await?;
            let desired: BTreeMap<String, Vec<i64>> = combinations.into_iter().collect();

            let existing = self
                .skus
                .list_by_product(context, None, input.product_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, tag, &error);
                    UsecaseError::Repository(error)
                })?;
            let existing_by_key: BTreeMap<String, ProductSku> = existing
                .into_iter()
                .map(|sku| (sku.selection_key.clone(), sku))
                .collect();

            let mut created_keys = Vec::new();
            for key in desired.keys() {
                if !existing_by_key.contains_key(key) {
                    created_keys.push(key.clone());
                }
            }

            let mut retained_keys = Vec::new();
            let mut removed_skus = Vec::new();
            for (key, sku) in &existing_by_key {
                if desired.contains_key(key) {
                    retained_keys.push(key.clone());
                    continue;
                }

                let holdings = self
                    .inventories
                    .list_by_sku(
                        context,
                        None,
                        sku.id,
                        Pagination {
                            limit: 1_000,
                            offset: 0,
                        },
                    )
                    .await
                    .map_err(|error| {
                        self.log_repository_error(context, tag, &error);
                        UsecaseError::Repository(error)
                    })?;
                let stock_on_hand: i64 = holdings.iter().map(|item| item.stock).sum();

                removed_skus.push(RemovedSku {
                    sku_id: sku.id,
                    selection_key: key.clone(),
                    stock_on_hand,
                });
            }

            let destroys_stock = removed_skus.iter().any(|sku| sku.stock_on_hand > 0);

            let plan = SkuRegenerationPlan {
                created_keys: created_keys.clone(),
                removed_skus: removed_skus.clone(),
                retained_keys,
                destroys_stock,
            };

            let plan_meta = Meta::from([
                ("product_id".into(), input.product_id.into()),
                ("created_count".into(), (created_keys.len() as u64).into()),
                ("removed_count".into(), (removed_skus.len() as u64).into()),
                ("destroys_stock".into(), destroys_stock.into()),
            ]);
            self.logger.info(
                context,
                tag,
                "computed the SKU regeneration plan",
                Some(&plan_meta),
            );

            if input.dry_run {
                return Ok(plan);
            }

            if destroys_stock {
                self.logger.warn(
                    context,
                    tag,
                    "SKU regeneration will destroy stock because a removed SKU is still held somewhere",
                    Some(&plan_meta),
                );
            }

            let skus = Arc::clone(&self.skus);
            let scoped = context.clone();
            let product_id = input.product_id;
            let desired_for_tx = desired.clone();
            let removed_for_tx = removed_skus.clone();

            let result = self
                .transactor
                .with_tx(move |executor| {
                    let skus = Arc::clone(&skus);
                    let scoped = scoped.clone();
                    let desired = desired_for_tx.clone();
                    let removed = removed_for_tx.clone();
                    Box::pin(async move {
                        for sku in &removed {
                            skus.delete(&scoped, Some(executor), sku.sku_id).await?;
                        }

                        let mut created = Vec::new();
                        for (key, selection_ids) in &desired {
                            let exists = skus
                                .find_by_selection_key(&scoped, Some(executor), product_id, key)
                                .await?;
                            if exists.is_none() {
                                let sku = skus
                                    .create(
                                        &scoped,
                                        Some(executor),
                                        CreateArgs {
                                            product_id,
                                            selection_key: key.clone(),
                                            code: None,
                                            variant_selection_ids: selection_ids.clone(),
                                            created_by: scoped.actor_id(),
                                        },
                                    )
                                    .await?;
                                created.push(sku);
                            }
                        }

                        Ok::<_, UsecaseError>(created)
                    })
                })
                .await;

            match result {
                Ok(created) => {
                    let meta = Meta::from([
                        ("product_id".into(), input.product_id.into()),
                        ("created_count".into(), (created.len() as u64).into()),
                        ("removed_count".into(), (removed_skus.len() as u64).into()),
                    ]);
                    self.logger
                        .info(context, tag, "SKU regeneration applied", Some(&meta));

                    Ok(SkuRegenerationPlan {
                        created_keys,
                        removed_skus,
                        retained_keys: plan.retained_keys,
                        destroys_stock,
                    })
                }
                Err(error) => {
                    self.log_usecase_error(context, tag, &error);
                    Err(error)
                }
            }
        })
    }
}

/// Keeps the imported set type used by the diff explicit.
#[allow(dead_code)]
type _KeySet = BTreeSet<String>;
