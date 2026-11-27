use std::sync::Arc;

use crate::{
    application::shared::permissions::{PERMISSION_CREATE, PERMISSION_LIST},
    domain::{
        contracts::{
            repository::{
                Pagination,
                permission::{CreateArgs, ListArgs, PermissionRepository},
            },
            utility::Logger,
            validator::PermissionValidator,
        },
        models::{
            AppContext, RepositoryError, UsecaseError, ValidatorError, entity::Permission,
            logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            access::permission::{CreatePermission, ListPermissions, ManagePermission},
            shared::PermissionGuard,
        },
    },
};

pub struct PermissionManagement {
    permissions: Arc<dyn PermissionRepository>,
    permission_guard: Arc<dyn PermissionGuard>,
    validator: Arc<dyn PermissionValidator>,
    logger: Arc<dyn Logger>,
}

impl PermissionManagement {
    pub fn new(
        permissions: Arc<dyn PermissionRepository>,
        permission_guard: Arc<dyn PermissionGuard>,
        validator: Arc<dyn PermissionValidator>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            permissions,
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

impl ManagePermission for PermissionManagement {
    fn create_permission<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreatePermission,
    ) -> UsecaseFuture<'a, Permission> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PERMISSION_CREATE])
                .await?;

            if let Err(error) = self.validator.slug(context, &input.slug) {
                self.log_validation_error(context, "application/access/permission/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.name(context, &input.name) {
                self.log_validation_error(context, "application/access/permission/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.description(context, &input.description) {
                self.log_validation_error(context, "application/access/permission/create", &error);
                return Err(UsecaseError::Validation(error));
            }

            match self
                .permissions
                .create(
                    context,
                    None,
                    CreateArgs {
                        slug: input.slug,
                        name: input.name,
                        description: input.description,
                    },
                )
                .await
            {
                Ok(permission) => {
                    let meta = Meta::from([("permission_id".into(), permission.id.into())]);
                    self.logger.info(
                        context,
                        "application/access/permission/create",
                        "permission created",
                        Some(&meta),
                    );
                    Ok(permission)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/access/permission/create",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_permissions<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListPermissions,
    ) -> UsecaseFuture<'a, Vec<Permission>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[PERMISSION_LIST])
                .await?;

            self.permissions
                .list(
                    context,
                    None,
                    ListArgs {
                        name_contains: request.name_contains,
                        pagination: Pagination {
                            limit: request.page.limit,
                            offset: request.page.offset,
                        },
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/access/permission/list",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }
}
