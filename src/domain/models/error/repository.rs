use std::{error::Error as StdError, fmt};

/// Persistence failures with generic and auth-specific classifications.
///
/// Operational variants retain their native cause; classification variants are
/// safe to map to stable diagnostic codes.
#[derive(Debug)]
pub enum RepositoryError {
    BadArgs,
    NotFound,
    Conflict,
    BadState,
    Timeout {
        source: Box<dyn StdError + Send + Sync + 'static>,
    },
    Failure {
        source: Box<dyn StdError + Send + Sync + 'static>,
    },
    Unimplemented,

    RoleNotFound,
    RoleSlugConflict,

    PermissionNotFound,
    PermissionSlugConflict,

    RolePermissionNotFound,
    RolePermissionConflict,

    UserNotFound,
    UserRoleNotFound,

    BaseAuthNotFound,
    BaseAuthUsernameConflict,
    BaseAuthEmailConflict,
    BaseAuthPhoneConflict,
    BaseAuthUserConflict,
}

impl RepositoryError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::BadArgs => "BAD_ARGS",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::BadState => "BAD_STATE",
            Self::Timeout { .. } => "TIMEOUT",
            Self::Failure { .. } => "FAILURE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::RoleNotFound => "ROLE_NOT_FOUND",
            Self::RoleSlugConflict => "ROLE_SLUG_CONFLICT",
            Self::PermissionNotFound => "PERMISSION_NOT_FOUND",
            Self::PermissionSlugConflict => "PERMISSION_SLUG_CONFLICT",
            Self::RolePermissionNotFound => "ROLE_PERMISSION_NOT_FOUND",
            Self::RolePermissionConflict => "ROLE_PERMISSION_CONFLICT",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::UserRoleNotFound => "USER_ROLE_NOT_FOUND",
            Self::BaseAuthNotFound => "BASE_AUTH_NOT_FOUND",
            Self::BaseAuthUsernameConflict => "BASE_AUTH_USERNAME_CONFLICT",
            Self::BaseAuthEmailConflict => "BASE_AUTH_EMAIL_CONFLICT",
            Self::BaseAuthPhoneConflict => "BASE_AUTH_PHONE_CONFLICT",
            Self::BaseAuthUserConflict => "BASE_AUTH_USER_CONFLICT",
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BadArgs => "invalid arguments",
            Self::NotFound => "record was not found",
            Self::Conflict => "record conflicts with existing data",
            Self::BadState => "invalid state",
            Self::Timeout { .. } => "repository operation timed out",
            Self::Failure { .. } => "repository operation failed",
            Self::Unimplemented => "operation is not implemented",
            Self::RoleNotFound => "role was not found",
            Self::RoleSlugConflict => "role slug conflicts with existing data",
            Self::PermissionNotFound => "permission was not found",
            Self::PermissionSlugConflict => "permission slug conflicts with existing data",
            Self::RolePermissionNotFound => "role permission grant was not found",
            Self::RolePermissionConflict => "role permission grant already exists",
            Self::UserNotFound => "user was not found",
            Self::UserRoleNotFound => "user role was not found",
            Self::BaseAuthNotFound => "base authentication record was not found",
            Self::BaseAuthUsernameConflict => "username conflicts with existing data",
            Self::BaseAuthEmailConflict => "email conflicts with existing data",
            Self::BaseAuthPhoneConflict => "phone conflicts with existing data",
            Self::BaseAuthUserConflict => "user already has base authentication data",
        })
    }
}

impl StdError for RepositoryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Timeout { source } | Self::Failure { source } => {
                Some(source.as_ref() as &(dyn StdError + 'static))
            }
            _ => None,
        }
    }
}
