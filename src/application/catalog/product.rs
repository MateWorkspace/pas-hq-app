use std::sync::Arc;

use crate::{
    application::shared::permissions::{
        PRODUCT_CREATE, PRODUCT_DELETE, PRODUCT_GET, PRODUCT_LIST, PRODUCT_UPDATE,
    },
    domain::{
        contracts::{
            repository::{
                Pagination,
                product::{CreateArgs, ListArgs, ProductRepository, UpdateArgs},
            },
            utility::Logger,
            validator::ProductValidator,
        },
        models::{
            AppContext, RepositoryError, UsecaseError, ValidatorError, entity::Product,
            logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            catalog::product::{CreateProduct, ListProducts, ManageProduct, UpdateProduct},
            shared::PermissionGuard,
        },
    },
};

pub struct ProductManagement {
    products: Arc<dyn ProductRepository>,
    permission_guard: Arc<dyn PermissionGuard>,
    validator: Arc<dyn ProductValidator>,
    logger: Arc<dyn Logger>,
}

impl ProductManagement {
    pub fn new(
        products: Arc<dyn ProductRepository>,
        permission_guard: Arc<dyn PermissionGuard>,
        validator: Arc<dyn ProductValidator>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            products,
            permission_guard,
            validator,
            logger,
        }
    }

    fn log_repository_error(&self, context: &AppContext, tag: &str, error: &RepositoryError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    fn log_validation_error(&self, context: &AppContext, tag: &str, error: &ValidatorError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }
}

impl ManageProduct for ProductManagement {
    fn create_product<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateProduct,
    ) -> UsecaseFuture<'a, Product> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRODUCT_CREATE])
                .await?;

            if let Err(error) = self.validator.brand(context, &input.brand) {
                self.log_validation_error(context, "application/catalog/product/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.model(context, &input.model) {
                self.log_validation_error(context, "application/catalog/product/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.manufacturer(context, &input.manufacturer) {
                self.log_validation_error(context, "application/catalog/product/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.description(context, &input.description) {
                self.log_validation_error(context, "application/catalog/product/create", &error);
                return Err(UsecaseError::Validation(error));
            }

            match self
                .products
                .create(
                    context,
                    None,
                    CreateArgs {
                        brand: input.brand,
                        model: input.model,
                        manufacturer: input.manufacturer,
                        description: input.description,
                        created_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(product) => {
                    let meta = Meta::from([("product_id".into(), product.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/product/create",
                        "product created",
                        Some(&meta),
                    );
                    Ok(product)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/product/create",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn update_product<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateProduct,
    ) -> UsecaseFuture<'a, Product> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRODUCT_UPDATE])
                .await?;

            if let Some(brand) = &input.brand
                && let Err(error) = self.validator.brand(context, brand)
            {
                self.log_validation_error(context, "application/catalog/product/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(model) = &input.model
                && let Err(error) = self.validator.model(context, model)
            {
                self.log_validation_error(context, "application/catalog/product/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(manufacturer) = &input.manufacturer
                && let Err(error) = self.validator.manufacturer(context, manufacturer)
            {
                self.log_validation_error(context, "application/catalog/product/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(description) = &input.description
                && let Err(error) = self.validator.description(context, description)
            {
                self.log_validation_error(context, "application/catalog/product/update", &error);
                return Err(UsecaseError::Validation(error));
            }

            match self
                .products
                .update(
                    context,
                    None,
                    input.id,
                    UpdateArgs {
                        brand: input.brand,
                        model: input.model,
                        manufacturer: input.manufacturer,
                        description: input.description,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(product) => {
                    let meta = Meta::from([("product_id".into(), product.id.into())]);
                    self.logger.info(
                        context,
                        "application/catalog/product/update",
                        "product updated",
                        Some(&meta),
                    );
                    Ok(product)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/product/update",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_product<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRODUCT_DELETE])
                .await?;

            match self.products.delete(context, None, id).await {
                Ok(()) => {
                    let meta = Meta::from([("product_id".into(), id.into())]);
                    self.logger.warn(
                        context,
                        "application/catalog/product/delete",
                        "product deleted; its variants, SKUs, prices, and stock were cascaded",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/catalog/product/delete",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_products<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListProducts,
    ) -> UsecaseFuture<'a, Vec<Product>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRODUCT_LIST])
                .await?;

            self.products
                .list(
                    context,
                    None,
                    ListArgs {
                        brand_contains: request.brand_contains,
                        name_contains: request.name_contains,
                        pagination: Pagination {
                            limit: request.page.limit,
                            offset: request.page.offset,
                        },
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/catalog/product/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn get_product<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, Product> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PRODUCT_GET])
                .await?;

            match self.products.find_by_id(context, None, id).await {
                Ok(Some(product)) => Ok(product),
                Ok(None) => {
                    self.logger.error(
                        context,
                        "application/catalog/product/get",
                        "product was not found",
                        Some(&Meta::from([("product_id".into(), id.into())])),
                    );
                    Err(UsecaseError::Repository(RepositoryError::ProductNotFound))
                }
                Err(error) => {
                    self.log_repository_error(context, "application/catalog/product/get", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}
