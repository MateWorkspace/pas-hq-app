use std::sync::Arc;

use crate::domain::{
    contracts::{
        repository::user::{UpdateArgs, UserRepository},
        utility::Logger,
        validator::UserValidator,
    },
    models::{
        AppContext, RepositoryError, UsecaseError, ValidatorError, entity::User, logger::Meta,
    },
    usecases::{
        UsecaseFuture,
        profile::account::{ManageAccount, UpdateAccount},
    },
};

/// A signed-in user editing their own record.
///
/// The target user is always `context.actor_id()`; there is no permission guard
/// because the operation only touches the caller's own account.
pub struct AccountManagement {
    users: Arc<dyn UserRepository>,
    validator: Arc<dyn UserValidator>,
    logger: Arc<dyn Logger>,
}

impl AccountManagement {
    pub fn new(
        users: Arc<dyn UserRepository>,
        validator: Arc<dyn UserValidator>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            users,
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

    fn actor(&self, context: &AppContext, tag: &str) -> Result<i64, UsecaseError> {
        match context.actor_id() {
            Some(id) => Ok(id),
            None => {
                self.logger.error(
                    context,
                    tag,
                    "profile operation rejected because the context has no authenticated claims",
                    None,
                );
                Err(UsecaseError::Unauthorized)
            }
        }
    }
}

impl ManageAccount for AccountManagement {
    fn get_account<'a>(&'a self, context: &'a AppContext) -> UsecaseFuture<'a, User> {
        Box::pin(async move {
            let user_id = self.actor(context, "application/profile/account/get")?;

            match self.users.find_by_id(context, None, user_id).await {
                Ok(Some(user)) => Ok(user),
                Ok(None) => {
                    self.logger.error(
                        context,
                        "application/profile/account/get",
                        "the signed-in user's record was not found",
                        Some(&Meta::from([("user_id".into(), user_id.into())])),
                    );
                    Err(UsecaseError::Repository(RepositoryError::UserNotFound))
                }
                Err(error) => {
                    self.log_repository_error(context, "application/profile/account/get", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn update_account<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateAccount,
    ) -> UsecaseFuture<'a, User> {
        Box::pin(async move {
            let user_id = self.actor(context, "application/profile/account/update")?;

            if let Some(name) = &input.name
                && let Err(error) = self.validator.name(context, name)
            {
                self.log_validation_error(context, "application/profile/account/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(bio) = &input.bio
                && let Err(error) = self.validator.bio(context, bio)
            {
                self.log_validation_error(context, "application/profile/account/update", &error);
                return Err(UsecaseError::Validation(error));
            }

            match self
                .users
                .update(
                    context,
                    None,
                    user_id,
                    UpdateArgs {
                        role_id: None,
                        name: input.name,
                        bio: input.bio,
                        avatar_path: input.avatar_path,
                        is_active: None,
                        suspended_until: None,
                        clear_suspended_until: false,
                        updated_by: Some(user_id),
                    },
                )
                .await
            {
                Ok(user) => {
                    let meta = Meta::from([("user_id".into(), user.id.into())]);
                    self.logger.info(
                        context,
                        "application/profile/account/update",
                        "account updated",
                        Some(&meta),
                    );
                    Ok(user)
                }
                Err(error) => {
                    self.log_repository_error(
                        context,
                        "application/profile/account/update",
                        &error,
                    );
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}
