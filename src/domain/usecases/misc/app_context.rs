use uuid::Uuid;

use crate::domain::models::{AccessClaims, AppContext};

use super::super::UsecaseFuture;

/// Builds the AppContext a presentation handler passes down.
///
/// Contexts are created before identity is known, so each constructor takes the
/// actor label it needs. `create_authenticated` is the single place that turns
/// validated claims into a user context, so the actor convention cannot drift
/// between the bearer and API key paths.
pub trait CreateAppContext: Send + Sync {
    /// Build a context with an explicit actor, trace, and optional claims.
    fn create<'a>(
        &'a self,
        actor: Option<String>,
        trace_id: Option<Uuid>,
        claims: Option<AccessClaims>,
    ) -> UsecaseFuture<'a, AppContext>;

    /// Build an unauthenticated context whose actor is `open`.
    fn create_open<'a>(&'a self, trace_id: Option<Uuid>) -> UsecaseFuture<'a, AppContext>;

    /// Build an internal context whose actor is `system`.
    fn create_system<'a>(&'a self, trace_id: Option<Uuid>) -> UsecaseFuture<'a, AppContext>;

    /// Build a user context from validated claims: the actor becomes the user's
    /// name and the claims travel with the context.
    fn create_authenticated<'a>(
        &'a self,
        claims: AccessClaims,
        trace_id: Option<Uuid>,
    ) -> UsecaseFuture<'a, AppContext>;
}
