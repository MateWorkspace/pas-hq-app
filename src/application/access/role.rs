use std::sync::Arc;

use crate::{
    application::shared::permissions::{
        ROLE_CREATE, ROLE_DELETE, ROLE_GRANT, ROLE_LIST, ROLE_UPDATE,
    },
    domain::{
        contracts::{
            repository::{
                Pagination,
                permission::PermissionRepository,
                role::{CreateArgs, ListArgs, RoleRepository, UpdateArgs},
                role_permission::{GrantArgs, RolePermissionRepository},
            },
            utility::Logger,
            validator::RoleValidator,
        },
        models::{
            AppContext, RepositoryError, UsecaseError, ValidatorError, entity::Role, logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            access::role::{CreateRole, GrantPermission, ListRoles, ManageRole, UpdateRole},
            shared::PermissionGuard,
        },
    },
};

pub struct RoleManagement {
    roles: Arc<dyn RoleRepository>,
    role_permissions: Arc<dyn RolePermissionRepository>,
    permissions: Arc<dyn PermissionRepository>,
    permission_guard: Arc<dyn PermissionGuard>,
    validator: Arc<dyn RoleValidator>,
    logger: Arc<dyn Logger>,
}

impl RoleManagement {
    pub fn new(
        roles: Arc<dyn RoleRepository>,
        role_permissions: Arc<dyn RolePermissionRepository>,
        permissions: Arc<dyn PermissionRepository>,
        permission_guard: Arc<dyn PermissionGuard>,
        validator: Arc<dyn RoleValidator>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            roles,
            role_permissions,
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

impl ManageRole for RoleManagement {
    fn create_role<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateRole,
    ) -> UsecaseFuture<'a, Role> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[ROLE_CREATE])
                .await?;

            if let Err(error) = self.validator.slug(context, &input.slug) {
                self.log_validation_error(context, "application/access/role/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.name(context, &input.name) {
                self.log_validation_error(context, "application/access/role/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.description(context, &input.description) {
                self.log_validation_error(context, "application/access/role/create", &error);
                return Err(UsecaseError::Validation(error));
            }

            let result = self
                .roles
                .create(
                    context,
                    None,
                    CreateArgs {
                        slug: input.slug,
                        name: input.name,
                        description: input.description,
                        is_default: input.is_default,
                        created_by: context.actor_id(),
                    },
                )
                .await;

            match result {
                Ok(role) => {
                    let meta = Meta::from([("role_id".into(), role.id.into())]);
                    self.logger.info(
                        context,
                        "application/access/role/create",
                        "role created",
                        Some(&meta),
                    );
                    Ok(role)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/access/role/create", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn update_role<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateRole,
    ) -> UsecaseFuture<'a, Role> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[ROLE_UPDATE])
                .await?;

            if let Some(name) = &input.name
                && let Err(error) = self.validator.name(context, name)
            {
                self.log_validation_error(context, "application/access/role/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(description) = &input.description
                && let Err(error) = self.validator.description(context, description)
            {
                self.log_validation_error(context, "application/access/role/update", &error);
                return Err(UsecaseError::Validation(error));
            }

            let result = self
                .roles
                .update(
                    context,
                    None,
                    input.id,
                    UpdateArgs {
                        name: input.name,
                        description: input.description,
                        is_default: input.is_default,
                        updated_by: context.actor_id(),
                    },
                )
                .await;

            match result {
                Ok(role) => {
                    let meta = Meta::from([("role_id".into(), role.id.into())]);
                    self.logger.info(
                        context,
                        "application/access/role/update",
                        "role updated",
                        Some(&meta),
                    );
                    Ok(role)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/access/role/update", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_role<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[ROLE_DELETE])
                .await?;

            match self.roles.delete(context, None, id).await {
                Ok(()) => {
                    let meta = Meta::from([("role_id".into(), id.into())]);
                    self.logger.info(
                        context,
                        "application/access/role/delete",
                        "role deleted",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(context, "application/access/role/delete", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_roles<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListRoles,
    ) -> UsecaseFuture<'a, Vec<Role>> {
        Box::pin(async move {
            self.permission_guard.require(context, &[ROLE_LIST]).await?;

            self.roles
                .list(
                    context,
                    None,
                    ListArgs {
                        is_default: request.is_default,
                        name_contains: request.name_contains,
                        pagination: Pagination {
                            limit: request.page.limit,
                            offset: request.page.offset,
                        },
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/access/role/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn get_role_permissions<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, Vec<String>> {
        Box::pin(async move {
            self.permission_guard.require(context, &[ROLE_LIST]).await?;

            // A role's grants are most useful as slugs, which is also what the
            // permission guard compares against.
            let grants = self
                .role_permissions
                .list_by_role(
                    context,
                    None,
                    id,
                    Pagination {
                        limit: 500,
                        offset: 0,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/access/role/get_permissions",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })?;

            let mut slugs = Vec::with_capacity(grants.len());
            for grant in grants {
                match self
                    .permissions
                    .find_by_id(context, None, grant.permission_id)
                    .await
                {
                    Ok(Some(permission)) => slugs.push(permission.slug),
                    Ok(None) => {
                        self.logger.warn(
                            context,
                            "application/access/role/get_permissions",
                            "a granted permission no longer exists and was skipped",
                            Some(&Meta::from([(
                                "permission_id".into(),
                                grant.permission_id.into(),
                            )])),
                        );
                    }
                    Err(error) => {
                        self.log_repository_error(
                            context,
                            "application/access/role/get_permissions",
                            &error,
                        );
                        return Err(UsecaseError::Repository(error));
                    }
                }
            }

            Ok(slugs)
        })
    }

    fn grant_permission<'a>(
        &'a self,
        context: &'a AppContext,
        input: GrantPermission,
    ) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[ROLE_GRANT])
                .await?;

            match self
                .role_permissions
                .grant(
                    context,
                    None,
                    GrantArgs {
                        role_id: input.role_id,
                        permission_id: input.permission_id,
                        created_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(grant) => {
                    let meta = Meta::from([
                        ("role_id".into(), input.role_id.into()),
                        ("permission_id".into(), input.permission_id.into()),
                        ("role_permission_id".into(), grant.id.into()),
                    ]);
                    self.logger.info(
                        context,
                        "application/access/role/grant_permission",
                        "permission granted to role",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/access/role/grant_permission",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn revoke_permission<'a>(
        &'a self,
        context: &'a AppContext,
        input: GrantPermission,
    ) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[ROLE_GRANT])
                .await?;

            match self
                .role_permissions
                .revoke(context, None, input.role_id, input.permission_id)
                .await
            {
                Ok(()) => {
                    let meta = Meta::from([
                        ("role_id".into(), input.role_id.into()),
                        ("permission_id".into(), input.permission_id.into()),
                    ]);
                    self.logger.info(
                        context,
                        "application/access/role/revoke_permission",
                        "permission revoked from role",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/access/role/revoke_permission",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}
