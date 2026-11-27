use uuid::Uuid;

use crate::domain::models::{AccessClaims, AppContext};

use super::super::UsecaseFuture;

/// Builds the AppContext a presentation handler passes down.
///
/// Contexts are created before identity is known, so each constructor takes the
/// actor label it needs. `create_system` is for internal work, and
/// `create_open` for unauthenticated requests such as sign-in.
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
}
