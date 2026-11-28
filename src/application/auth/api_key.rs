use std::sync::Arc;

use crate::domain::{
    contracts::{
        caching::ApiKeyClaimsCache,
        repository::api_key::ApiKeyRepository,
        utility::{ApiKey, Logger},
    },
    models::{AccessClaims, AppContext, UsecaseError, logger::Meta},
    usecases::{UsecaseFuture, auth::api_key::ApiKeyAuth},
};

/// Resolves an API key into claims, using the digest-keyed cache.
pub struct ApiKeyAuthentication {
    api_keys: Arc<dyn ApiKeyRepository>,
    cache: Arc<dyn ApiKeyClaimsCache>,
    api_key: Arc<dyn ApiKey>,
    logger: Arc<dyn Logger>,
}

impl ApiKeyAuthentication {
    pub fn new(
        api_keys: Arc<dyn ApiKeyRepository>,
        cache: Arc<dyn ApiKeyClaimsCache>,
        api_key: Arc<dyn ApiKey>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            api_keys,
            cache,
            api_key,
            logger,
        }
    }

    fn reject(&self, context: &AppContext, tag: &str, message: &str) -> UsecaseError {
        self.logger.warn(context, tag, message, None);
        UsecaseError::Unauthorized
    }
}

impl ApiKeyAuth for ApiKeyAuthentication {
    fn validate_api_key<'a>(
        &'a self,
        context: &'a AppContext,
        raw_key: &'a str,
    ) -> UsecaseFuture<'a, AccessClaims> {
        Box::pin(async move {
            let tag = "application/auth/api_key/validate";

            if !self.api_key.is_well_formed(raw_key) {
                return Err(self.reject(
                    context,
                    tag,
                    "api key validation rejected because the key format is invalid",
                ));
            }

            let key_hash = match self.api_key.hash(raw_key) {
                Ok(hash) => hash,
                Err(error) => {
                    self.logger.error(
                        context,
                        tag,
                        "api key validation failed because the key could not be hashed",
                        Some(&Meta::from([("error_code".into(), error.code().into())])),
                    );
                    return Err(UsecaseError::ApiKey(error));
                }
            };

            if let Some(claims) = self.cache.get(context, &key_hash).await {
                return Ok(claims);
            }

            let auth = self
                .api_keys
                .find_auth_by_hash(context, None, &key_hash)
                .await
                .map_err(|error| {
                    self.logger.error(
                        context,
                        tag,
                        "api key validation failed while reading the key",
                        Some(&Meta::from([("error_code".into(), error.code().into())])),
                    );
                    UsecaseError::Repository(error)
                })?;

            let Some(auth) = auth else {
                return Err(self.reject(
                    context,
                    tag,
                    "api key validation rejected because the key is unknown",
                ));
            };

            if !auth.key_is_active {
                return Err(self.reject(
                    context,
                    tag,
                    "api key validation rejected because the key is disabled",
                ));
            }

            if !auth.is_usable(chrono::Utc::now()) {
                return Err(self.reject(
                    context,
                    tag,
                    "api key validation rejected because the owner is inactive or suspended",
                ));
            }

            let claims = auth.claims();
            self.cache.set(context, &key_hash, &claims).await;

            Ok(claims)
        })
    }
}
