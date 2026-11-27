use std::{error::Error as StdError, fmt};

/// Validation failures with stable diagnostic codes.
///
/// Variants describe auth field rules owned by the domain. They carry no rejected
/// value, transport status, or credential material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidatorError {
    UsernameTooShort,
    UsernameTooLong,
    UsernameInvalid,

    PasswordTooShort,
    PasswordTooLong,
    PasswordMissingUppercase,
    PasswordMissingLowercase,
    PasswordMissingDigit,
    PasswordMissingSymbol,

    RoleSlugTooShort,
    RoleSlugTooLong,
    RoleSlugInvalid,
    RoleNameTooShort,
    RoleNameTooLong,
    RoleNameInvalid,
    RoleDescriptionTooLong,

    PermissionSlugTooShort,
    PermissionSlugTooLong,
    PermissionSlugInvalid,
    PermissionNameTooShort,
    PermissionNameTooLong,
    PermissionNameInvalid,
    PermissionDescriptionTooLong,

    UserNameTooShort,
    UserNameTooLong,
    UserNameInvalid,
    UserBioTooLong,

    LocationNameTooShort,
    LocationNameTooLong,
    LocationNameInvalid,
    LocationDescriptionTooLong,

    ProductBrandInvalid,
    ProductModelInvalid,
    ProductManufacturerInvalid,
    ProductDescriptionTooLong,
}

impl ValidatorError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UsernameTooShort => "AUTH_USERNAME_TOO_SHORT",
            Self::UsernameTooLong => "AUTH_USERNAME_TOO_LONG",
            Self::UsernameInvalid => "AUTH_USERNAME_INVALID",
            Self::PasswordTooShort => "AUTH_PASSWORD_TOO_SHORT",
            Self::PasswordTooLong => "AUTH_PASSWORD_TOO_LONG",
            Self::PasswordMissingUppercase => "AUTH_PASSWORD_MISSING_UPPERCASE",
            Self::PasswordMissingLowercase => "AUTH_PASSWORD_MISSING_LOWERCASE",
            Self::PasswordMissingDigit => "AUTH_PASSWORD_MISSING_DIGIT",
            Self::PasswordMissingSymbol => "AUTH_PASSWORD_MISSING_SYMBOL",
            Self::RoleSlugTooShort => "ROLE_SLUG_TOO_SHORT",
            Self::RoleSlugTooLong => "ROLE_SLUG_TOO_LONG",
            Self::RoleSlugInvalid => "ROLE_SLUG_INVALID",
            Self::RoleNameTooShort => "ROLE_NAME_TOO_SHORT",
            Self::RoleNameTooLong => "ROLE_NAME_TOO_LONG",
            Self::RoleNameInvalid => "ROLE_NAME_INVALID",
            Self::RoleDescriptionTooLong => "ROLE_DESCRIPTION_TOO_LONG",
            Self::PermissionSlugTooShort => "PERMISSION_SLUG_TOO_SHORT",
            Self::PermissionSlugTooLong => "PERMISSION_SLUG_TOO_LONG",
            Self::PermissionSlugInvalid => "PERMISSION_SLUG_INVALID",
            Self::PermissionNameTooShort => "PERMISSION_NAME_TOO_SHORT",
            Self::PermissionNameTooLong => "PERMISSION_NAME_TOO_LONG",
            Self::PermissionNameInvalid => "PERMISSION_NAME_INVALID",
            Self::PermissionDescriptionTooLong => "PERMISSION_DESCRIPTION_TOO_LONG",
            Self::UserNameTooShort => "USER_NAME_TOO_SHORT",
            Self::UserNameTooLong => "USER_NAME_TOO_LONG",
            Self::UserNameInvalid => "USER_NAME_INVALID",
            Self::UserBioTooLong => "USER_BIO_TOO_LONG",
            Self::LocationNameTooShort => "LOCATION_NAME_TOO_SHORT",
            Self::LocationNameTooLong => "LOCATION_NAME_TOO_LONG",
            Self::LocationNameInvalid => "LOCATION_NAME_INVALID",
            Self::LocationDescriptionTooLong => "LOCATION_DESCRIPTION_TOO_LONG",
            Self::ProductBrandInvalid => "PRODUCT_BRAND_INVALID",
            Self::ProductModelInvalid => "PRODUCT_MODEL_INVALID",
            Self::ProductManufacturerInvalid => "PRODUCT_MANUFACTURER_INVALID",
            Self::ProductDescriptionTooLong => "PRODUCT_DESCRIPTION_TOO_LONG",
        }
    }
}

impl fmt::Display for ValidatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UsernameTooShort => "username is too short",
            Self::UsernameTooLong => "username is too long",
            Self::UsernameInvalid => "username contains unsupported characters",
            Self::PasswordTooShort => "password is too short",
            Self::PasswordTooLong => "password is too long",
            Self::PasswordMissingUppercase => "password must contain an uppercase letter",
            Self::PasswordMissingLowercase => "password must contain a lowercase letter",
            Self::PasswordMissingDigit => "password must contain a digit",
            Self::PasswordMissingSymbol => "password must contain a symbol",
            Self::RoleSlugTooShort => "role slug is too short",
            Self::RoleSlugTooLong => "role slug is too long",
            Self::RoleSlugInvalid => "role slug format is invalid",
            Self::RoleNameTooShort => "role name is too short",
            Self::RoleNameTooLong => "role name is too long",
            Self::RoleNameInvalid => "role name is invalid",
            Self::RoleDescriptionTooLong => "role description is too long",
            Self::PermissionSlugTooShort => "permission slug is too short",
            Self::PermissionSlugTooLong => "permission slug is too long",
            Self::PermissionSlugInvalid => "permission slug format is invalid",
            Self::PermissionNameTooShort => "permission name is too short",
            Self::PermissionNameTooLong => "permission name is too long",
            Self::PermissionNameInvalid => "permission name is invalid",
            Self::PermissionDescriptionTooLong => "permission description is too long",
            Self::UserNameTooShort => "user name is too short",
            Self::UserNameTooLong => "user name is too long",
            Self::UserNameInvalid => "user name is invalid",
            Self::UserBioTooLong => "user bio is too long",
            Self::LocationNameTooShort => "location name is too short",
            Self::LocationNameTooLong => "location name is too long",
            Self::LocationNameInvalid => "location name is invalid",
            Self::LocationDescriptionTooLong => "location description is too long",
            Self::ProductBrandInvalid => "product brand is invalid",
            Self::ProductModelInvalid => "product model is invalid",
            Self::ProductManufacturerInvalid => "product manufacturer is invalid",
            Self::ProductDescriptionTooLong => "product description is too long",
        })
    }
}

impl StdError for ValidatorError {}
