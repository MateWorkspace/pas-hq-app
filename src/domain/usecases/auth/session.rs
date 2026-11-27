use crate::domain::models::{AccessClaims, AppContext};

use super::super::UsecaseFuture;

/// User session lifecycle: sign-in, access-token validation, and refresh.
///
/// Sign-in and refresh run under an unauthenticated ("open") context. Validation
/// runs under a system context because the caller has no identity until it
/// succeeds; the returned claims are then used to build the user context.
pub trait Session: Send + Sync {
    /// Exchange username/password credentials for an access and refresh pair.
    ///
    /// A wrong password and an unknown username fail identically, so the call never
    /// reveals which usernames exist. A suspended or inactive account is rejected
    /// after the password is verified.
    fn sign_in<'a>(
        &'a self,
        context: &'a AppContext,
        input: SignInInput,
    ) -> UsecaseFuture<'a, SessionTokens>;

    /// Verify an access token and return its claims. Permissions are not embedded;
    /// callers resolve them from the role when they authorize an operation.
    fn validate_access<'a>(
        &'a self,
        context: &'a AppContext,
        access_token: &'a str,
    ) -> UsecaseFuture<'a, AccessClaims>;

    /// Exchange a valid refresh token for a new access and refresh pair.
    ///
    /// Live identity, role, and account state are rechecked; a refresh never
    /// restores access on its own.
    fn refresh<'a>(
        &'a self,
        context: &'a AppContext,
        input: RefreshInput,
    ) -> UsecaseFuture<'a, SessionTokens>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshInput {
    pub refresh_token: String,
}

/// A freshly issued token pair and the identity it belongs to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub claims: AccessClaims,
}
