use std::sync::Arc;

use crate::{
    application::shared::permissions::{
        APIKEY_CREATE, APIKEY_DELETE, APIKEY_LIST, APIKEY_ROTATE, APIKEY_UPDATE,
    },
    domain::{
        contracts::{
            caching::ApiKeyClaimsCache,
            repository::api_key::{ApiKeyRepository, CreateArgs, RotateArgs},
            utility::{ApiKey as ApiKeyGenerator, Logger},
            validator::ApiKeyValidator,
        },
        models::{
            AppContext, RepositoryError, UsecaseError, ValidatorError, entity::ApiKey, logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            access::api_key::{CreateApiKey, ListUserApiKeys, ManageApiKey, SetApiKeyActive},
            auth::api_key::IssuedApiKey,
            shared::PermissionGuard,
        },
    },
};

/// Administrator API key management across users.
pub struct ApiKeyManagement {
    api_keys: Arc<dyn ApiKeyRepository>,
    cache: Arc<dyn ApiKeyClaimsCache>,
    generator: Arc<dyn ApiKeyGenerator>,
    validator: Arc<dyn ApiKeyValidator>,
    permission_guard: Arc<dyn PermissionGuard>,
    logger: Arc<dyn Logger>,
}

impl ApiKeyManagement {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        api_keys: Arc<dyn ApiKeyRepository>,
        cache: Arc<dyn ApiKeyClaimsCache>,
        generator: Arc<dyn ApiKeyGenerator>,
        validator: Arc<dyn ApiKeyValidator>,
        permission_guard: Arc<dyn PermissionGuard>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            api_keys,
            cache,
            generator,
            validator,
            permission_guard,
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

    async fn find(&self, context: &AppContext, id: i64, tag: &str) -> Result<ApiKey, UsecaseError> {
        match self.api_keys.find_by_id(context, None, id).await {
            Ok(Some(key)) => Ok(key),
            Ok(None) => {
                self.logger.error(
                    context,
                    tag,
                    "api key was not found",
                    Some(&Meta::from([("api_key_id".into(), id.into())])),
                );
                Err(UsecaseError::Repository(RepositoryError::ApiKeyNotFound))
            }
            Err(error) => {
                self.log_repository_error(context, tag, &error);
                Err(UsecaseError::Repository(error))
            }
        }
    }
}

impl ManageApiKey for ApiKeyManagement {
    fn list_api_keys<'a>(
        &'a self,
        context: &'a AppContext,
        page: crate::domain::usecases::Page,
    ) -> UsecaseFuture<'a, Vec<ApiKey>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[APIKEY_LIST])
                .await?;

            self.api_keys
                .list_all(
                    context,
                    None,
                    crate::domain::contracts::repository::Pagination {
                        limit: page.limit,
                        offset: page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/access/api_key/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn list_user_api_keys<'a>(
        &'a self,
        context: &'a AppContext,
        input: ListUserApiKeys,
    ) -> UsecaseFuture<'a, Vec<ApiKey>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[APIKEY_LIST])
                .await?;

            self.api_keys
                .list_by_user(
                    context,
                    None,
                    input.user_id,
                    crate::domain::contracts::repository::Pagination {
                        limit: input.page.limit,
                        offset: input.page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(
                        context,
                        "application/access/api_key/list_user",
                        &error,
                    );
                    UsecaseError::Repository(error)
                })
        })
    }

    fn create_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateApiKey,
    ) -> UsecaseFuture<'a, IssuedApiKey> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[APIKEY_CREATE])
                .await?;

            let tag = "application/access/api_key/create";

            if let Err(error) = self.validator.name(context, &input.name) {
                self.log_validation_error(context, tag, &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.description(context, &input.description) {
                self.log_validation_error(context, tag, &error);
                return Err(UsecaseError::Validation(error));
            }

            let generated = match self.generator.generate() {
                Ok(generated) => generated,
                Err(error) => {
                    self.logger.error(
                        context,
                        tag,
                        "api key creation failed because a key could not be generated",
                        Some(&Meta::from([("error_code".into(), error.code().into())])),
                    );
                    return Err(UsecaseError::ApiKey(error));
                }
            };

            match self
                .api_keys
                .create(
                    context,
                    None,
                    CreateArgs {
                        user_id: input.user_id,
                        name: input.name,
                        description: input.description,
                        redacted: generated.redacted,
                        hashed_key: generated.hash,
                        created_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(key) => {
                    let meta = Meta::from([
                        ("api_key_id".into(), key.id.into()),
                        ("user_id".into(), key.user_id.into()),
                    ]);
                    self.logger
                        .info(context, tag, "api key created", Some(&meta));
                    Ok(IssuedApiKey {
                        key,
                        raw: generated.raw,
                    })
                }
                Err(error) => {
                    self.log_repository_error(context, tag, &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn rotate_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, IssuedApiKey> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[APIKEY_ROTATE])
                .await?;

            let tag = "application/access/api_key/rotate";
            let key = self.find(context, id, tag).await?;

            let generated = match self.generator.generate() {
                Ok(generated) => generated,
                Err(error) => {
                    self.logger.error(
                        context,
                        tag,
                        "api key rotation failed because a key could not be generated",
                        Some(&Meta::from([("error_code".into(), error.code().into())])),
                    );
                    return Err(UsecaseError::ApiKey(error));
                }
            };

            match self
                .api_keys
                .rotate(
                    context,
                    None,
                    id,
                    RotateArgs {
                        redacted: generated.redacted,
                        hashed_key: generated.hash,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(rotated) => {
                    self.cache.remove(context, &key.hashed_key).await;
                    let meta = Meta::from([("api_key_id".into(), rotated.id.into())]);
                    self.logger.warn(
                        context,
                        tag,
                        "api key rotated; the previous key no longer authenticates",
                        Some(&meta),
                    );
                    Ok(IssuedApiKey {
                        key: rotated,
                        raw: generated.raw,
                    })
                }
                Err(error) => {
                    self.log_repository_error(context, tag, &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn set_api_key_active<'a>(
        &'a self,
        context: &'a AppContext,
        input: SetApiKeyActive,
    ) -> UsecaseFuture<'a, ApiKey> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[APIKEY_UPDATE])
                .await?;

            let tag = "application/access/api_key/set_active";
            let key = self.find(context, input.id, tag).await?;

            match self
                .api_keys
                .set_active(context, None, input.id, input.is_active, context.actor_id())
                .await
            {
                Ok(updated) => {
                    if !input.is_active {
                        self.cache.remove(context, &key.hashed_key).await;
                    }
                    let meta = Meta::from([
                        ("api_key_id".into(), updated.id.into()),
                        ("is_active".into(), input.is_active.into()),
                    ]);
                    self.logger
                        .warn(context, tag, "api key activation changed", Some(&meta));
                    Ok(updated)
                }
                Err(error) => {
                    self.log_repository_error(context, tag, &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_api_key<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[APIKEY_DELETE])
                .await?;

            let tag = "application/access/api_key/delete";

            match self.api_keys.delete(context, None, id).await {
                Ok(deleted) => {
                    self.cache.remove(context, &deleted.hashed_key).await;
                    let meta = Meta::from([("api_key_id".into(), id.into())]);
                    self.logger
                        .warn(context, tag, "api key deleted", Some(&meta));
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(context, tag, &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}
