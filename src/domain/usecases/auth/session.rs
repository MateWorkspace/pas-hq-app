use crate::domain::models::{AccessClaims, AppContext};

use super::super::UsecaseFuture;

/// User session lifecycle: sign-in, access-token validation, refresh, and profile.
///
/// Sign-in and refresh run under an unauthenticated ("open") context. Validation
/// runs under a system context because the caller has no identity until it
/// succeeds; the returned claims are then used to build the user context. Profile
/// reads run under an authenticated context and re-resolve live state.
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

    /// Read the caller's live identity and effective permissions.
    ///
    /// The context must carry validated claims; the target is always
    /// `context.actor_id()`, so no permission guard applies and no other user can
    /// be addressed. Identity, role, and account state are re-read from storage so
    /// a suspension, deactivation, or role change is corrected immediately instead
    /// of at access-token expiry.
    fn profile<'a>(&'a self, context: &'a AppContext) -> UsecaseFuture<'a, SessionProfile>;
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

/// A freshly issued token pair and the session it belongs to.
///
/// Permissions travel with the pair so a client needs exactly one call after
/// sign-in or refresh; there is no follow-up profile request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub profile: SessionProfile,
}

/// Live identity plus its effective permission slugs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionProfile {
    pub user: SessionUser,
    pub permissions: Vec<String>,
}

/// The identity carried by an access token, resolved from live storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionUser {
    pub id: i64,
    pub name: String,
    pub role_id: i64,
    pub role_name: String,
    /// Stable role slug used for permission resolution.
    pub role: String,
}

impl SessionUser {
    pub fn claims(&self) -> AccessClaims {
        AccessClaims {
            user_id: self.id,
            name: self.name.clone(),
            role_id: self.role_id,
            role_name: self.role_name.clone(),
            role: self.role.clone(),
        }
    }
}
