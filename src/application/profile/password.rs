use std::sync::Arc;

use crate::domain::{
    contracts::{
        repository::base_auth::{BaseAuthRepository, UpdateArgs as BaseAuthUpdateArgs},
        utility::{Logger, Password},
        validator::AuthValidator,
    },
    models::{
        AppContext, PasswordError, RepositoryError, UsecaseError, ValidatorError, logger::Meta,
    },
    usecases::{
        UsecaseFuture,
        profile::password::{ChangePassword, ChangePasswordInput},
    },
};

/// A signed-in user changing their own password.
pub struct PasswordChange {
    base_auths: Arc<dyn BaseAuthRepository>,
    auth_validator: Arc<dyn AuthValidator>,
    password: Arc<dyn Password>,
    logger: Arc<dyn Logger>,
}

impl PasswordChange {
    pub fn new(
        base_auths: Arc<dyn BaseAuthRepository>,
        auth_validator: Arc<dyn AuthValidator>,
        password: Arc<dyn Password>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            base_auths,
            auth_validator,
            password,
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

impl ChangePassword for PasswordChange {
    fn change_password<'a>(
        &'a self,
        context: &'a AppContext,
        input: ChangePasswordInput,
    ) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            let Some(user_id) = context.actor_id() else {
                self.logger.error(
                    context,
                    "application/profile/password/change",
                    "password change rejected because the context has no authenticated claims",
                    None,
                );
                return Err(UsecaseError::Unauthorized);
            };

            if let Err(error) = self.auth_validator.password(context, &input.new_password) {
                self.log_validation_error(context, "application/profile/password/change", &error);
                return Err(UsecaseError::Validation(error));
            }

            let auth = match self
                .base_auths
                .find_by_user_id(context, None, user_id)
                .await
            {
                Ok(Some(auth)) => auth,
                Ok(None) => {
                    self.logger.error(
                        context,
                        "application/profile/password/change",
                        "credentials were not found for the signed-in user",
                        Some(&Meta::from([("user_id".into(), user_id.into())])),
                    );
                    return Err(UsecaseError::Repository(RepositoryError::BaseAuthNotFound));
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/profile/password/change",
                        &error,
                    );
                    return Err(UsecaseError::Repository(error));
                }
            };

            if let Err(error) = self
                .password
                .compare(&auth.password_hash, &input.current_password)
            {
                let meta = Meta::from([("error_code".into(), error.code().into())]);
                self.logger.warn(
                    context,
                    "application/profile/password/change",
                    "password change rejected because the current password did not match",
                    Some(&meta),
                );
                return Err(UsecaseError::Password(error));
            }

            let password_hash = match self.password.hash(&input.new_password) {
                Ok(hash) => hash,
                Err(error) => {
                    let meta = Meta::from([("error_code".into(), error.code().into())]);
                    self.logger.error(
                        context,
                        "application/profile/password/change",
                        &error.to_string(),
                        Some(&meta),
                    );
                    return Err(UsecaseError::Password(error));
                }
            };

            match self
                .base_auths
                .update(
                    context,
                    None,
                    auth.id,
                    BaseAuthUpdateArgs {
                        username: None,
                        password_hash: Some(password_hash),
                        email: None,
                        phone: None,
                        is_email_verified: None,
                        is_phone_verified: None,
                        updated_by: Some(user_id),
                    },
                )
                .await
            {
                Ok(_) => {
                    let meta = Meta::from([("user_id".into(), user_id.into())]);
                    self.logger.info(
                        context,
                        "application/profile/password/change",
                        "password changed",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/profile/password/change",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}

#[allow(dead_code)]
fn _assert_password_error(_: PasswordError) {}
