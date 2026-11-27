use crate::domain::models::AppContext;

use super::super::UsecaseFuture;

/// Authorizes an operation against the acting role's granted permissions.
///
/// Every required permission must be present; there is no role-name or actor
/// bypass. Permissions come from the role's grant set, which is served from cache,
/// so a check does not add a database round trip on the hot path.
pub trait PermissionGuard: Send + Sync {
    /// Fail with `BAD_ARGS` for an empty requirement list and `FORBIDDEN` when any
    /// permission is missing or the context has no authenticated claims.
    fn require<'a>(
        &'a self,
        context: &'a AppContext,
        required: &'a [&'a str],
    ) -> UsecaseFuture<'a, ()>;
}
