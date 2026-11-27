use std::sync::Arc;

use crate::{
    application::shared::permissions::{SELECTION_MANAGE, VARIANT_MANAGE},
    domain::{
        contracts::{
            repository::{
                product_variant::{CreateArgs, ProductVariantRepository, UpdateArgs},
                product_variant_selection::{
                    CreateArgs as SelectionCreateArgs, ProductVariantSelectionRepository,
                    UpdateArgs as SelectionUpdateArgs,
                },
            },
            utility::Logger,
        },
        models::{
            AppContext, RepositoryError, UsecaseError,
            entity::{ProductVariant, ProductVariantSelection},
            logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            catalog::variant::{
                CreateSelection, CreateVariant, ManageVariant, ManageVariantSelection,
                UpdateSelection, UpdateVariant,
            },
            shared::PermissionGuard,
        },
    },
};

/// Variant dimensions and their selectable values.
///
/// Any write invalidates the product's SKUs; the caller must run SKU regeneration
/// afterwards. This usecase does not regenerate, so a design change and the
/// reconciliation stay separate decisions.
pub struct VariantManagement {
    variants: Arc<dyn ProductVariantRepository>,
    selections: Arc<dyn ProductVariantSelectionRepository>,
    permission_guard: Arc<dyn PermissionGuard>,
    logger: Arc<dyn Logger>,
}

impl VariantManagement {
    pub fn new(
        variants: Arc<dyn ProductVariantRepository>,
        selections: Arc<dyn ProductVariantSelectionRepository>,
        permission_guard: Arc<dyn PermissionGuard>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            variants,
            selections,
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

impl ManageVariant for VariantManagement {
    fn create_variant<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateVariant,
    ) -> UsecaseFuture<'a, ProductVariant> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[VARIANT_MANAGE])
                .await?;

            match self
                .variants
                .create(
                    context,
                    None,
                    CreateArgs {
                        product_id: input.product_id,
                        name: input.name,
                        created_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(variant) => {
                    let meta = Meta::from([("variant_id".into(), variant.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/variant/create",
                        "variant created; SKU regeneration is required",
                        Some(&meta),
                    );
                    Ok(variant)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/variant/create",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn update_variant<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateVariant,
    ) -> UsecaseFuture<'a, ProductVariant> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[VARIANT_MANAGE])
                .await?;

            match self
                .variants
                .update(
                    context,
                    None,
                    input.id,
                    UpdateArgs {
                        name: input.name,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(variant) => {
                    let meta = Meta::from([("variant_id".into(), variant.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/variant/update",
                        "variant updated; SKU regeneration is required",
                        Some(&meta),
                    );
                    Ok(variant)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/variant/update",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_variant<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[VARIANT_MANAGE])
                .await?;

            match self.variants.delete(context, None, id).await {
                Ok(()) => {
                    let meta = Meta::from([("variant_id".into(), id.into())]);
                    self.logger.warn(
                        context,
                        "application/catalog/variant/delete",
                        "variant deleted; affected SKUs still exist and must be regenerated",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/variant/delete",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_variants<'a>(
        &'a self,
        context: &'a AppContext,
        product_id: i64,
    ) -> UsecaseFuture<'a, Vec<ProductVariant>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[VARIANT_MANAGE])
                .await?;

            self.variants
                .list_by_product(context, None, product_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/catalog/variant/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }
}

impl ManageVariantSelection for VariantManagement {
    fn create_selection<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateSelection,
    ) -> UsecaseFuture<'a, ProductVariantSelection> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SELECTION_MANAGE])
                .await?;

            match self
                .selections
                .create(
                    context,
                    None,
                    SelectionCreateArgs {
                        product_variant_id: input.product_variant_id,
                        name: input.name,
                        description: input.description,
                        created_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(selection) => {
                    let meta = Meta::from([("selection_id".into(), selection.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/selection/create",
                        "variant selection created; SKU regeneration is required",
                        Some(&meta),
                    );
                    Ok(selection)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/selection/create",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn update_selection<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateSelection,
    ) -> UsecaseFuture<'a, ProductVariantSelection> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SELECTION_MANAGE])
                .await?;

            match self
                .selections
                .update(
                    context,
                    None,
                    input.id,
                    SelectionUpdateArgs {
                        name: input.name,
                        description: input.description,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(selection) => {
                    let meta = Meta::from([("selection_id".into(), selection.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/selection/update",
                        "variant selection updated; SKU regeneration is required",
                        Some(&meta),
                    );
                    Ok(selection)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/selection/update",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_selection<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SELECTION_MANAGE])
                .await?;

            match self.selections.delete(context, None, id).await {
                Ok(()) => {
                    let meta = Meta::from([("selection_id".into(), id.into())]);
                    self.logger.warn(
                        context,
                        "application/catalog/selection/delete",
                        "variant selection deleted; affected SKUs still exist and must be regenerated",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/selection/delete",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_selections<'a>(
        &'a self,
        context: &'a AppContext,
        product_variant_id: i64,
    ) -> UsecaseFuture<'a, Vec<ProductVariantSelection>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[SELECTION_MANAGE])
                .await?;

            self.selections
                .list_by_variant(context, None, product_variant_id)
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/catalog/selection/list",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }
}
