use std::sync::Arc;

use crate::domain::{
    contracts::{
        caching::ApiKeyClaimsCache,
        repository::api_key::{ApiKeyRepository, CreateArgs, RotateArgs},
        utility::{ApiKey as ApiKeyGenerator, Logger},
        validator::ApiKeyValidator,
    },
    models::{
        ApiKeyError, AppContext, RepositoryError, UsecaseError, ValidatorError, entity::ApiKey,
        logger::Meta,
    },
    usecases::{
        Page, UsecaseFuture,
        auth::api_key::IssuedApiKey,
        profile::api_key::{CreateOwnApiKey, ManageOwnApiKey},
    },
};

/// Self-service API key management for the signed-in user.
///
/// The owner is always `context.actor_id()`, so a caller cannot touch another
/// user's keys and no permission guard applies.
pub struct OwnApiKeyManagement {
    api_keys: Arc<dyn ApiKeyRepository>,
    cache: Arc<dyn ApiKeyClaimsCache>,
    generator: Arc<dyn ApiKeyGenerator>,
    validator: Arc<dyn ApiKeyValidator>,
    logger: Arc<dyn Logger>,
}

impl OwnApiKeyManagement {
    pub fn new(
        api_keys: Arc<dyn ApiKeyRepository>,
        cache: Arc<dyn ApiKeyClaimsCache>,
        generator: Arc<dyn ApiKeyGenerator>,
        validator: Arc<dyn ApiKeyValidator>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            api_keys,
            cache,
            generator,
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

    fn owner(&self, context: &AppContext, tag: &str) -> Result<i64, UsecaseError> {
        match context.actor_id() {
            Some(id) => Ok(id),
            None => {
                self.logger.error(
                    context,
                    tag,
                    "api key management rejected because the context has no authenticated claims",
                    None,
                );
                Err(UsecaseError::Unauthorized)
            }
        }
    }

    /// Confirm the key belongs to the caller.
    async fn owned(
        &self,
        context: &AppContext,
        id: i64,
        tag: &str,
    ) -> Result<ApiKey, UsecaseError> {
        match self.api_keys.find_by_id(context, None, id).await {
            Ok(Some(key)) if Some(key.user_id) == context.actor_id() => Ok(key),
            Ok(Some(_)) => {
                self.logger.warn(
                    context,
                    tag,
                    "api key management rejected because the key belongs to another user",
                    Some(&Meta::from([("api_key_id".into(), id.into())])),
                );
                Err(UsecaseError::Forbidden)
            }
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

impl ManageOwnApiKey for OwnApiKeyManagement {
    fn list_own_api_keys<'a>(
        &'a self,
        context: &'a AppContext,
        page: Page,
    ) -> UsecaseFuture<'a, Vec<ApiKey>> {
        Box::pin(async move {
            let tag = "application/profile/api_key/list";
            let user_id = self.owner(context, tag)?;

            self.api_keys
                .list_by_user(
                    context,
                    None,
                    user_id,
                    crate::domain::contracts::repository::Pagination {
                        limit: page.limit,
                        offset: page.offset,
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(context, tag, &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn create_own_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateOwnApiKey,
    ) -> UsecaseFuture<'a, IssuedApiKey> {
        Box::pin(async move {
            let tag = "application/profile/api_key/create";
            let user_id = self.owner(context, tag)?;

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
                        user_id,
                        name: input.name,
                        description: input.description,
                        redacted: generated.redacted,
                        hashed_key: generated.hash,
                        created_by: Some(user_id),
                    },
                )
                .await
            {
                Ok(key) => {
                    let meta = Meta::from([("api_key_id".into(), key.id.into())]);
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

    fn rotate_own_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, IssuedApiKey> {
        Box::pin(async move {
            let tag = "application/profile/api_key/rotate";
            let key = self.owned(context, id, tag).await?;

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

            let old_hash = key.hashed_key;
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
                    self.cache.remove(context, &old_hash).await;
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

    fn set_own_api_key_active<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
        is_active: bool,
    ) -> UsecaseFuture<'a, ApiKey> {
        Box::pin(async move {
            let tag = "application/profile/api_key/set_active";
            let key = self.owned(context, id, tag).await?;
            let hashed_key = key.hashed_key.clone();

            match self
                .api_keys
                .set_active(context, None, id, is_active, context.actor_id())
                .await
            {
                Ok(updated) => {
                    if !is_active {
                        self.cache.remove(context, &hashed_key).await;
                    }
                    let meta = Meta::from([
                        ("api_key_id".into(), updated.id.into()),
                        ("is_active".into(), is_active.into()),
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

    fn delete_own_api_key<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            let tag = "application/profile/api_key/delete";
            // Ownership is checked before the delete; the removed row carries the
            // digest used for cache invalidation.
            self.owned(context, id, tag).await?;

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

#[allow(dead_code)]
fn _assert_api_key_error(_: ApiKeyError) {}
