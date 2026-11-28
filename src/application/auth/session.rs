use std::sync::Arc;

use crate::domain::{
    contracts::{
        repository::{
            base_auth::BaseAuthRepository, role::RoleRepository,
            role_permission::RolePermissionRepository, user::UserRepository,
        },
        utility::{Logger, Password, Token},
    },
    models::{
        AccessClaims, AppContext, PasswordError, RefreshClaims, UsecaseError, entity::User,
        logger::Meta,
    },
    usecases::{
        UsecaseFuture,
        auth::session::{
            RefreshInput, Session, SessionProfile, SessionTokens, SessionUser, SignInInput,
        },
    },
};

/// Credential sign-in, access validation, refresh, and session profile reads.
pub struct SessionUsecase {
    users: Arc<dyn UserRepository>,
    base_auths: Arc<dyn BaseAuthRepository>,
    roles: Arc<dyn RoleRepository>,
    role_permissions: Arc<dyn RolePermissionRepository>,
    password: Arc<dyn Password>,
    token: Arc<dyn Token>,
    logger: Arc<dyn Logger>,
}

impl SessionUsecase {
    pub fn new(
        users: Arc<dyn UserRepository>,
        base_auths: Arc<dyn BaseAuthRepository>,
        roles: Arc<dyn RoleRepository>,
        role_permissions: Arc<dyn RolePermissionRepository>,
        password: Arc<dyn Password>,
        token: Arc<dyn Token>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            users,
            base_auths,
            roles,
            role_permissions,
            password,
            token,
            logger,
        }
    }

    fn log_repository_error(
        &self,
        context: &AppContext,
        tag: &str,
        error: &crate::domain::models::RepositoryError,
    ) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    /// Resolve the live identity and role behind a user id.
    async fn profile_for(
        &self,
        context: &AppContext,
        user_id: i64,
        tag: &str,
    ) -> Result<SessionProfile, UsecaseError> {
        let user = match self.users.find_by_id(context, None, user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                self.logger.error(
                    context,
                    tag,
                    "the authenticated user no longer exists",
                    Some(&Meta::from([("user_id".into(), user_id.into())])),
                );
                return Err(UsecaseError::Unauthorized);
            }
            Err(error) => {
                self.log_repository_error(context, tag, &error);
                return Err(UsecaseError::Repository(error));
            }
        };

        self.ensure_active(context, &user, tag)?;
        let session_user = self.session_user_for_user(context, &user, tag).await?;
        let permissions = self
            .permissions_for(context, &session_user.role, tag)
            .await?;

        Ok(SessionProfile {
            user: session_user,
            permissions,
        })
    }

    async fn session_user_for_user(
        &self,
        context: &AppContext,
        user: &User,
        tag: &str,
    ) -> Result<SessionUser, UsecaseError> {
        let role = match self.roles.find_by_id(context, None, user.role_id).await {
            Ok(Some(role)) => role,
            Ok(None) => {
                self.logger.error(
                    context,
                    tag,
                    "the user's role no longer exists",
                    Some(&Meta::from([("role_id".into(), user.role_id.into())])),
                );
                return Err(UsecaseError::Unauthorized);
            }
            Err(error) => {
                self.log_repository_error(context, tag, &error);
                return Err(UsecaseError::Repository(error));
            }
        };

        Ok(SessionUser {
            id: user.id,
            name: user.name.clone(),
            role_id: role.id,
            role_name: role.name,
            role: role.slug,
        })
    }

    /// Effective permission slugs for a role. The repository decorator caches this
    /// lookup, so a repeated profile read does not reach the database.
    async fn permissions_for(
        &self,
        context: &AppContext,
        role_slug: &str,
        tag: &str,
    ) -> Result<Vec<String>, UsecaseError> {
        match self
            .role_permissions
            .find_by_role_slug(context, None, role_slug)
            .await
        {
            Ok(permissions) => Ok(permissions),
            Err(error) => {
                self.log_repository_error(context, tag, &error);
                Err(UsecaseError::Repository(error))
            }
        }
    }

    fn ensure_active(
        &self,
        context: &AppContext,
        user: &User,
        tag: &str,
    ) -> Result<(), UsecaseError> {
        if !user.is_active {
            self.logger.warn(
                context,
                tag,
                "rejected a session for an inactive user",
                Some(&Meta::from([("user_id".into(), user.id.into())])),
            );
            return Err(UsecaseError::Unauthorized);
        }

        if let Some(until) = user.suspended_until
            && until > chrono::Utc::now()
        {
            self.logger.warn(
                context,
                tag,
                "rejected a session for a suspended user",
                Some(&Meta::from([("user_id".into(), user.id.into())])),
            );
            return Err(UsecaseError::Unauthorized);
        }

        Ok(())
    }

    fn issue(&self, profile: &SessionProfile) -> Result<SessionTokens, UsecaseError> {
        let claims = profile.user.claims();
        let refresh_claims = RefreshClaims {
            user_id: claims.user_id,
            name: claims.name.clone(),
        };
        let access_token = self
            .token
            .generate_access(&claims)
            .map_err(UsecaseError::Token)?;
        let refresh_token = self.token.generate_refresh(&refresh_claims)?;

        Ok(SessionTokens {
            access_token,
            refresh_token,
            profile: profile.clone(),
        })
    }
}

impl Session for SessionUsecase {
    fn sign_in<'a>(
        &'a self,
        context: &'a AppContext,
        input: SignInInput,
    ) -> UsecaseFuture<'a, SessionTokens> {
        Box::pin(async move {
            let auth = match self
                .base_auths
                .find_by_username(context, None, &input.username)
                .await
            {
                Ok(Some(auth)) => auth,
                Ok(None) => {
                    self.logger.warn(
                        context,
                        "application/auth/session/sign_in",
                        "sign-in failed because the username is unknown",
                        None,
                    );
                    return Err(UsecaseError::Password(PasswordError::Mismatch));
                }
                Err(error) => {
                    self.log_repository_error(context, "application/auth/session/sign_in", &error);
                    return Err(UsecaseError::Repository(error));
                }
            };

            if let Err(error) = self.password.compare(&auth.password_hash, &input.password) {
                let meta = Meta::from([("error_code".into(), error.code().into())]);
                self.logger.warn(
                    context,
                    "application/auth/session/sign_in",
                    "sign-in failed because the password did not match",
                    Some(&meta),
                );
                return Err(UsecaseError::Password(error));
            }

            let profile = self
                .profile_for(context, auth.user_id, "application/auth/session/sign_in")
                .await?;

            let tokens = self.issue(&profile)?;

            let meta = Meta::from([("user_id".into(), profile.user.id.into())]);
            self.logger.info(
                context,
                "application/auth/session/sign_in",
                "user signed in",
                Some(&meta),
            );

            Ok(tokens)
        })
    }

    fn validate_access<'a>(
        &'a self,
        context: &'a AppContext,
        access_token: &'a str,
    ) -> UsecaseFuture<'a, AccessClaims> {
        Box::pin(async move {
            match self.token.validate_access(access_token) {
                Ok(claims) => Ok(claims),
                Err(error) => {
                    let meta = Meta::from([("error_code".into(), error.code().into())]);
                    self.logger.warn(
                        context,
                        "application/auth/session/validate_access",
                        "access token validation failed",
                        Some(&meta),
                    );
                    Err(UsecaseError::Token(error))
                }
            }
        })
    }

    fn refresh<'a>(
        &'a self,
        context: &'a AppContext,
        input: RefreshInput,
    ) -> UsecaseFuture<'a, SessionTokens> {
        Box::pin(async move {
            let refresh_claims = match self.token.validate_refresh(&input.refresh_token) {
                Ok(claims) => claims,
                Err(error) => {
                    let meta = Meta::from([("error_code".into(), error.code().into())]);
                    self.logger.warn(
                        context,
                        "application/auth/session/refresh",
                        "refresh token validation failed",
                        Some(&meta),
                    );
                    return Err(UsecaseError::Token(error));
                }
            };

            let profile = self
                .profile_for(
                    context,
                    refresh_claims.user_id,
                    "application/auth/session/refresh",
                )
                .await?;

            let tokens = self.issue(&profile)?;

            let meta = Meta::from([("user_id".into(), profile.user.id.into())]);
            self.logger.info(
                context,
                "application/auth/session/refresh",
                "session refreshed",
                Some(&meta),
            );

            Ok(tokens)
        })
    }

    fn profile<'a>(&'a self, context: &'a AppContext) -> UsecaseFuture<'a, SessionProfile> {
        Box::pin(async move {
            let Some(user_id) = context.actor_id() else {
                self.logger.error(
                    context,
                    "application/auth/session/profile",
                    "profile was requested without an authenticated context",
                    None,
                );
                return Err(UsecaseError::Unauthorized);
            };

            self.profile_for(context, user_id, "application/auth/session/profile")
                .await
        })
    }
}
