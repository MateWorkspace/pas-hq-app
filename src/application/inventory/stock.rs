use std::sync::Arc;

use crate::{
    application::shared::permissions::{
        MOVEMENT_LIST, MOVEMENT_TRANSFER, STOCK_ADJUST, STOCK_LIST,
    },
    domain::{
        contracts::{
            repository::{
                Pagination,
                inventory::{InventoryRepository, SetStockArgs},
                stock_movement::{RecordArgs, StockMovementRepository},
            },
            utility::Logger,
        },
        models::{
            AppContext, MovementReason, RepositoryError, UsecaseError, entity::Inventory,
            logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            inventory::{
                movement::{RecordMovement, TransferStock},
                stock::{
                    AdjustStock, AdjustStockInput, GetStock, ListStock, MovementByLocation,
                    MovementBySku, StockByLocation, StockBySku, StockKey,
                },
            },
            shared::PermissionGuard,
        },
    },
};

use crate::{
    domain::contracts::utility::Transactor, infrastructure::utility::transactor::PgTransactor,
};

pub struct StockManagement {
    inventories: Arc<dyn InventoryRepository>,
    movements: Arc<dyn StockMovementRepository>,
    transactor: Arc<PgTransactor>,
    permission_guard: Arc<dyn PermissionGuard>,
    logger: Arc<dyn Logger>,
}

impl StockManagement {
    pub fn new(
        inventories: Arc<dyn InventoryRepository>,
        movements: Arc<dyn StockMovementRepository>,
        transactor: Arc<PgTransactor>,
        permission_guard: Arc<dyn PermissionGuard>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            inventories,
            movements,
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
}

impl GetStock for StockManagement {
    fn get_stock<'a>(
        &'a self,
        context: &'a AppContext,
        input: StockKey,
    ) -> UsecaseFuture<'a, Option<Inventory>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[STOCK_LIST])
                .await?;

            self.inventories
                .find(context, None, input.location_id, input.sku_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/inventory/stock/get", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn list_stock_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        request: StockByLocation,
    ) -> UsecaseFuture<'a, Vec<Inventory>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[STOCK_LIST])
                .await?;

            self.inventories
                .list_by_location(
                    context,
                    None,
                    request.location_id,
                    Pagination {
                        limit: request.page.limit,
                        offset: request.page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/inventory/stock/list_by_location",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }

    fn list_stock_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        request: StockBySku,
    ) -> UsecaseFuture<'a, Vec<Inventory>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[STOCK_LIST])
                .await?;

            self.inventories
                .list_by_sku(
                    context,
                    None,
                    request.sku_id,
                    Pagination {
                        limit: request.page.limit,
                        offset: request.page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/inventory/stock/list_by_sku",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }
}

impl AdjustStock for StockManagement {
    fn adjust_stock<'a>(
        &'a self,
        context: &'a AppContext,
        input: AdjustStockInput,
    ) -> UsecaseFuture<'a, Inventory> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[STOCK_ADJUST])
                .await?;

            let tag = "application/inventory/stock/adjust";

            if input.quantity == 0 {
                self.logger.error(
                    context,
                    tag,
                    "stock adjustment rejected because the quantity is zero",
                    None,
                );
                return Err(UsecaseError::Validation(
                    crate::domain::models::ValidatorError::LocationNameInvalid,
                ));
            }

            let current = self
                .inventories
                .find(context, None, input.location_id, input.sku_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, tag, &error);
                    UsecaseError::Repository(error)
                })?
                .map_or(0, |inventory| inventory.stock);

            let next = current + input.quantity;
            if next < 0 {
                self.logger.warn(
                    context,
                    tag,
                    "stock adjustment rejected because it would drive stock negative",
                    Some(&Meta::from([
                        ("location_id".into(), input.location_id.into()),
                        ("sku_id".into(), input.sku_id.into()),
                        ("stock".into(), current.into()),
                        ("quantity".into(), input.quantity.into()),
                    ])),
                );
                return Err(UsecaseError::Repository(RepositoryError::BadArgs));
            }

            let inventories = Arc::clone(&self.inventories);
            let movements = Arc::clone(&self.movements);
            let scoped = context.clone();

            let result = self
                .transactor
                .with_tx(move |executor| {
                    let inventories = Arc::clone(&inventories);
                    let movements = Arc::clone(&movements);
                    let scoped = scoped.clone();
                    Box::pin(async move {
                        let inventory = inventories
                            .set_stock(
                                &scoped,
                                Some(executor),
                                SetStockArgs {
                                    location_id: input.location_id,
                                    sku_id: input.sku_id,
                                    stock: next,
                                    updated_by: scoped.actor_id(),
                                },
                            )
                            .await?;

                        movements
                            .record(
                                &scoped,
                                Some(executor),
                                RecordArgs {
                                    sku_id: Some(input.sku_id),
                                    sku_label: None,
                                    src_location_id: if input.quantity < 0 {
                                        Some(input.location_id)
                                    } else {
                                        None
                                    },
                                    src_location_name: None,
                                    dest_location_id: if input.quantity > 0 {
                                        Some(input.location_id)
                                    } else {
                                        None
                                    },
                                    dest_location_name: None,
                                    quantity: input.quantity.abs(),
                                    reason: input.reason,
                                },
                            )
                            .await?;

                        Ok::<_, UsecaseError>(inventory)
                    })
                })
                .await;

            match result {
                Ok(inventory) => {
                    let meta = Meta::from([
                        ("location_id".into(), input.location_id.into()),
                        ("sku_id".into(), input.sku_id.into()),
                        ("quantity".into(), input.quantity.into()),
                    ]);
                    self.logger
                        .info(context, tag, "stock adjusted and recorded", Some(&meta));
                    Ok(inventory)
                }
                Err(error) => {
                    self.log_usecase_error(context, tag, &error);
                    Err(error)
                }
            }
        })
    }
}

impl ListStock for StockManagement {
    fn list_stock_movements_by_sku<'a>(
        &'a self,
        context: &'a AppContext,
        request: MovementBySku,
    ) -> UsecaseFuture<'a, Vec<crate::domain::models::StockMovement>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[MOVEMENT_LIST])
                .await?;

            self.movements
                .list_by_sku(
                    context,
                    None,
                    request.sku_id,
                    Pagination {
                        limit: request.page.limit,
                        offset: request.page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/inventory/movement/list_by_sku",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }

    fn list_stock_movements_by_location<'a>(
        &'a self,
        context: &'a AppContext,
        request: MovementByLocation,
    ) -> UsecaseFuture<'a, Vec<crate::domain::models::StockMovement>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[MOVEMENT_LIST])
                .await?;

            self.movements
                .list_by_location(
                    context,
                    None,
                    request.location_id,
                    Pagination {
                        limit: request.page.limit,
                        offset: request.page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/inventory/movement/list_by_location",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }
}

impl RecordMovement for StockManagement {
    fn transfer_stock<'a>(
        &'a self,
        context: &'a AppContext,
        input: TransferStock,
    ) -> UsecaseFuture<'a, crate::domain::models::StockMovement> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[MOVEMENT_TRANSFER])
                .await?;

            let tag = "application/inventory/movement/transfer";

            if input.src_location_id == input.dest_location_id {
                self.logger.error(
                    context,
                    tag,
                    "transfer rejected because the source and destination are the same location",
                    None,
                );
                return Err(UsecaseError::Repository(RepositoryError::BadArgs));
            }
            if input.quantity <= 0 {
                self.logger.error(
                    context,
                    tag,
                    "transfer rejected because the quantity is not positive",
                    None,
                );
                return Err(UsecaseError::Repository(RepositoryError::BadArgs));
            }

            let source = self
                .inventories
                .find(context, None, input.src_location_id, input.sku_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, tag, &error);
                    UsecaseError::Repository(error)
                })?
                .map_or(0, |inventory| inventory.stock);

            if source < input.quantity {
                self.logger.warn(
                    context,
                    tag,
                    "transfer rejected because the source location does not hold enough stock",
                    Some(&Meta::from([
                        ("location_id".into(), input.src_location_id.into()),
                        ("sku_id".into(), input.sku_id.into()),
                        ("stock".into(), source.into()),
                        ("quantity".into(), input.quantity.into()),
                    ])),
                );
                return Err(UsecaseError::Repository(RepositoryError::BadArgs));
            }

            let destination = self
                .inventories
                .find(context, None, input.dest_location_id, input.sku_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, tag, &error);
                    UsecaseError::Repository(error)
                })?
                .map_or(0, |inventory| inventory.stock);

            let inventories = Arc::clone(&self.inventories);
            let movements = Arc::clone(&self.movements);
            let scoped = context.clone();

            let result = self
                .transactor
                .with_tx(move |executor| {
                    let inventories = Arc::clone(&inventories);
                    let movements = Arc::clone(&movements);
                    let scoped = scoped.clone();
                    Box::pin(async move {
                        inventories
                            .set_stock(
                                &scoped,
                                Some(executor),
                                SetStockArgs {
                                    location_id: input.src_location_id,
                                    sku_id: input.sku_id,
                                    stock: source - input.quantity,
                                    updated_by: scoped.actor_id(),
                                },
                            )
                            .await?;

                        inventories
                            .set_stock(
                                &scoped,
                                Some(executor),
                                SetStockArgs {
                                    location_id: input.dest_location_id,
                                    sku_id: input.sku_id,
                                    stock: destination + input.quantity,
                                    updated_by: scoped.actor_id(),
                                },
                            )
                            .await?;

                        let movement = movements
                            .record(
                                &scoped,
                                Some(executor),
                                RecordArgs {
                                    sku_id: Some(input.sku_id),
                                    sku_label: None,
                                    src_location_id: Some(input.src_location_id),
                                    src_location_name: None,
                                    dest_location_id: Some(input.dest_location_id),
                                    dest_location_name: None,
                                    quantity: input.quantity,
                                    reason: MovementReason::Transfer,
                                },
                            )
                            .await?;

                        Ok::<_, UsecaseError>(movement)
                    })
                })
                .await;

            match result {
                Ok(movement) => {
                    let meta = Meta::from([
                        ("sku_id".into(), input.sku_id.into()),
                        ("src_location_id".into(), input.src_location_id.into()),
                        ("dest_location_id".into(), input.dest_location_id.into()),
                        ("quantity".into(), input.quantity.into()),
                    ]);
                    self.logger
                        .info(context, tag, "stock transferred", Some(&meta));
                    Ok(movement)
                }
                Err(error) => {
                    self.log_usecase_error(context, tag, &error);
                    Err(error)
                }
            }
        })
    }
}
