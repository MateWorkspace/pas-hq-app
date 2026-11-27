use std::sync::Arc;

use crate::domain::{
    contracts::{repository::RolePermissionRepository, utility::Logger},
    models::{AppContext, UsecaseError, logger::Meta},
    usecases::{UsecaseFuture, shared::PermissionGuard},
};

/// Authorizes operations against the acting role's cached grant set.
pub struct RolePermissionGuard {
    role_permissions: Arc<dyn RolePermissionRepository>,
    logger: Arc<dyn Logger>,
}

impl RolePermissionGuard {
    pub fn new(
        role_permissions: Arc<dyn RolePermissionRepository>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            role_permissions,
            logger,
        }
    }

    fn reject(&self, context: &AppContext, tag: &str, message: &str) -> UsecaseError {
        self.logger.error(context, tag, message, None);
        UsecaseError::Forbidden
    }
}

impl PermissionGuard for RolePermissionGuard {
    fn require<'a>(
        &'a self,
        context: &'a AppContext,
        required: &'a [&'a str],
    ) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            if required.is_empty() {
                self.logger.error(
                    context,
                    "application/shared/permission_guard/require",
                    "permission guard was called without any required permission",
                    None,
                );
                return Err(UsecaseError::Validation(
                    crate::domain::models::ValidatorError::PermissionNameInvalid,
                ));
            }

            let Some(role_slug) = context.role_slug() else {
                return Err(self.reject(
                    context,
                    "application/shared/permission_guard/require",
                    "permission check rejected because the context has no authenticated claims",
                ));
            };

            let granted = self
                .role_permissions
                .find_by_role_slug(context, None, role_slug)
                .await
                .map_err(|error| {
                    self.logger.error(
                        context,
                        "application/shared/permission_guard/require",
                        "failed to resolve role permissions for the permission check",
                        Some(&Meta::from([("error_code".into(), error.code().into())])),
                    );
                    UsecaseError::Repository(error)
                })?;

            for permission in required {
                if !granted.iter().any(|value| value == permission) {
                    return Err(self.reject(
                        context,
                        "application/shared/permission_guard/require",
                        "permission check rejected because the role lacks a required permission",
                    ));
                }
            }

            Ok(())
        })
    }
}
