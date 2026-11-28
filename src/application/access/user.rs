use std::sync::Arc;

use crate::{
    application::shared::permissions::{
        USER_CREATE, USER_DELETE, USER_GET, USER_LIST, USER_UPDATE,
    },
    domain::{
        contracts::{
            repository::{
                Pagination,
                base_auth::{BaseAuthRepository, CreateArgs as BaseAuthCreateArgs},
                user::{CreateArgs, ListArgs, UpdateArgs, UserRepository},
            },
            utility::{Logger, Password},
            validator::{AuthValidator, UserValidator},
        },
        models::{
            AppContext, RepositoryError, UsecaseError, ValidatorError, entity::User, logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            access::user::{CreateUser, ListUsers, ManageUser, UpdateUser},
            shared::PermissionGuard,
        },
    },
};

use crate::{
    domain::contracts::{
        caching::ApiKeyClaimsCache, repository::api_key::ApiKeyRepository, utility::Transactor,
    },
    infrastructure::utility::transactor::PgTransactor,
};

pub struct UserManagement {
    users: Arc<dyn UserRepository>,
    base_auths: Arc<dyn BaseAuthRepository>,
    api_keys: Arc<dyn ApiKeyRepository>,
    api_key_cache: Arc<dyn ApiKeyClaimsCache>,
    transactor: Arc<PgTransactor>,
    permission_guard: Arc<dyn PermissionGuard>,
    auth_validator: Arc<dyn AuthValidator>,
    user_validator: Arc<dyn UserValidator>,
    password: Arc<dyn Password>,
    logger: Arc<dyn Logger>,
}

impl UserManagement {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        users: Arc<dyn UserRepository>,
        base_auths: Arc<dyn BaseAuthRepository>,
        api_keys: Arc<dyn ApiKeyRepository>,
        api_key_cache: Arc<dyn ApiKeyClaimsCache>,
        transactor: Arc<PgTransactor>,
        permission_guard: Arc<dyn PermissionGuard>,
        auth_validator: Arc<dyn AuthValidator>,
        user_validator: Arc<dyn UserValidator>,
        password: Arc<dyn Password>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            users,
            base_auths,
            api_keys,
            api_key_cache,
            transactor,
            permission_guard,
            auth_validator,
            user_validator,
            password,
            logger,
        }
    }

    fn log_repository_error(&self, context: &AppContext, tag: &str, error: &RepositoryError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    fn log_usecase_error(&self, context: &AppContext, tag: &str, error: &UsecaseError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    /// Drop cached claims for every key the user owns.
    async fn invalidate_user_api_keys(&self, context: &AppContext, user_id: i64, tag: &str) {
        match self
            .api_keys
            .list_by_user(
                context,
                None,
                user_id,
                Pagination {
                    limit: 1_000,
                    offset: 0,
                },
            )
            .await
        {
            Ok(keys) => {
                for key in keys {
                    self.api_key_cache.remove(context, &key.hashed_key).await;
                }
            }
            Err(error) => {
                self.logger.warn(
                    context,
                    tag,
                    "cached api key claims could not be invalidated after a user change",
                    Some(&Meta::from([("error_code".into(), error.code().into())])),
                );
            }
        }
    }

    fn log_validation_error(&self, context: &AppContext, tag: &str, error: &ValidatorError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }
}

impl ManageUser for UserManagement {
    fn create_user<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateUser,
    ) -> UsecaseFuture<'a, User> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[USER_CREATE])
                .await?;

            if let Err(error) = self.user_validator.name(context, &input.name) {
                self.log_validation_error(context, "application/access/user/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.user_validator.bio(context, &input.bio) {
                self.log_validation_error(context, "application/access/user/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.auth_validator.username(context, &input.username) {
                self.log_validation_error(context, "application/access/user/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.auth_validator.password(context, &input.password) {
                self.log_validation_error(context, "application/access/user/create", &error);
                return Err(UsecaseError::Validation(error));
            }

            let password_hash = match self.password.hash(&input.password) {
                Ok(hash) => hash,
                Err(error) => {
                    let meta = Meta::from([("error_code".into(), error.code().into())]);
                    self.logger.error(
                        context,
                        "application/access/user/create",
                        &error.to_string(),
                        Some(&meta),
                    );
                    return Err(UsecaseError::Password(error));
                }
            };

            let create_args = CreateArgs {
                role_id: input.role_id,
                name: input.name.clone(),
                bio: input.bio.clone(),
                avatar_path: input.avatar_path.clone(),
                is_active: input.is_active,
                created_by: context.actor_id(),
            };
            let base_args = BaseAuthCreateArgs {
                user_id: 0,
                username: input.username,
                password_hash,
                email: input.email,
                phone: input.phone,
                created_by: context.actor_id(),
            };

            let users = Arc::clone(&self.users);
            let base_auths = Arc::clone(&self.base_auths);
            let scoped = context.clone();

            let result = self
                .transactor
                .with_tx(move |executor| {
                    let users = Arc::clone(&users);
                    let base_auths = Arc::clone(&base_auths);
                    let scoped = scoped.clone();
                    let create_args = create_args.clone();
                    let base_args = base_args.clone();
                    Box::pin(async move {
                        let user = users.create(&scoped, Some(executor), create_args).await?;
                        base_auths
                            .create(
                                &scoped,
                                Some(executor),
                                BaseAuthCreateArgs {
                                    user_id: user.id,
                                    ..base_args
                                },
                            )
                            .await?;
                        Ok::<_, UsecaseError>(user)
                    })
                })
                .await;

            match result {
                Ok(user) => {
                    let meta = Meta::from([("user_id".into(), user.id.into())]);
                    self.logger.info(
                        context,
                        "application/access/user/create",
                        "user and credentials created",
                        Some(&meta),
                    );
                    Ok(user)
                }
                Err(error) => {
                    self.log_usecase_error(context, "application/access/user/create", &error);
                    Err(error)
                }
            }
        })
    }

    fn update_user<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateUser,
    ) -> UsecaseFuture<'a, User> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[USER_UPDATE])
                .await?;

            if let Some(name) = &input.name
                && let Err(error) = self.user_validator.name(context, name)
            {
                self.log_validation_error(context, "application/access/user/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(bio) = &input.bio
                && let Err(error) = self.user_validator.bio(context, bio)
            {
                self.log_validation_error(context, "application/access/user/update", &error);
                return Err(UsecaseError::Validation(error));
            }

            // A cached key carries the user's role, so any identity-bearing change
            // must drop the cached claims instead of waiting for the TTL.
            let invalidates_credentials = input.role_id.is_some()
                || input.is_active.is_some()
                || input.suspended_until.is_some()
                || input.clear_suspended_until;

            match self
                .users
                .update(
                    context,
                    None,
                    input.id,
                    UpdateArgs {
                        role_id: input.role_id,
                        name: input.name,
                        bio: input.bio,
                        avatar_path: input.avatar_path,
                        is_active: input.is_active,
                        suspended_until: input.suspended_until,
                        clear_suspended_until: input.clear_suspended_until,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(user) => {
                    if invalidates_credentials {
                        self.invalidate_user_api_keys(
                            context,
                            user.id,
                            "application/access/user/update",
                        )
                        .await;
                    }
                    let meta = Meta::from([("user_id".into(), user.id.into())]);
                    self.logger.info(
                        context,
                        "application/access/user/update",
                        "user updated",
                        Some(&meta),
                    );
                    Ok(user)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/access/user/update", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_user<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[USER_DELETE])
                .await?;

            let users = Arc::clone(&self.users);
            let base_auths = Arc::clone(&self.base_auths);
            let api_keys = Arc::clone(&self.api_keys);
            let scoped = context.clone();

            let result = self
                .transactor
                .with_tx(move |executor| {
                    let users = Arc::clone(&users);
                    let base_auths = Arc::clone(&base_auths);
                    let api_keys = Arc::clone(&api_keys);
                    let scoped = scoped.clone();
                    Box::pin(async move {
                        if let Some(auth) = base_auths
                            .find_by_user_id(&scoped, Some(executor), id)
                            .await?
                        {
                            base_auths.delete(&scoped, Some(executor), auth.id).await?;
                        }
                        let key_hashes = api_keys
                            .delete_by_user_id(&scoped, Some(executor), id)
                            .await?;
                        users.delete(&scoped, Some(executor), id).await?;
                        Ok::<_, UsecaseError>(key_hashes)
                    })
                })
                .await;

            match result {
                Ok(key_hashes) => {
                    for key_hash in &key_hashes {
                        self.api_key_cache.remove(context, key_hash).await;
                    }
                    let meta = Meta::from([
                        ("user_id".into(), id.into()),
                        ("api_key_count".into(), (key_hashes.len() as u64).into()),
                    ]);
                    self.logger.info(
                        context,
                        "application/access/user/delete",
                        "user, credentials, and api keys deleted",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_usecase_error(context, "application/access/user/delete", &error);
                    Err(error)
                }
            }
        })
    }

    fn list_users<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListUsers,
    ) -> UsecaseFuture<'a, Vec<User>> {
        Box::pin(async move {
            self.permission_guard.require(context, &[USER_LIST]).await?;

            self.users
                .list(
                    context,
                    None,
                    ListArgs {
                        role_id: request.role_id,
                        is_active: request.is_active,
                        name_contains: request.name_contains,
                        pagination: Pagination {
                            limit: request.page.limit,
                            offset: request.page.offset,
                        },
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/access/user/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn get_user<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, User> {
        Box::pin(async move {
            self.permission_guard.require(context, &[USER_GET]).await?;

            match self.users.find_by_id(context, None, id).await {
                Ok(Some(user)) => Ok(user),
                Ok(None) => {
                    self.logger.error(
                        context,
                        "application/access/user/get",
                        "user was not found",
                        Some(&Meta::from([("user_id".into(), id.into())])),
                    );
                    Err(UsecaseError::Repository(RepositoryError::UserNotFound))
                }
                Err(error) => {
                    self.log_repository_error(context, "application/access/user/get", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}
