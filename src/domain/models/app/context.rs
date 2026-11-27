use uuid::Uuid;

use super::super::entity::AccessClaims;

/// Request-scoped metadata propagated through contracts and usecases.
///
/// `actor` is an opaque caller label. `claims` is present only after a successful
/// access-token validation and is the sole source of the acting identity; it may be
/// absent and must not be treated as proof of authorization on its own.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AppContext {
    pub actor: Option<String>,
    pub trace_id: Option<Uuid>,
    pub execution_id: Option<Uuid>,
    pub claims: Option<AccessClaims>,
}

impl AppContext {
    /// The acting user id, present only for an authenticated context.
    pub fn actor_id(&self) -> Option<i64> {
        self.claims.as_ref().map(|claims| claims.user_id)
    }

    /// The acting role slug, present only for an authenticated context.
    pub fn role_slug(&self) -> Option<&str> {
        self.claims.as_ref().map(|claims| claims.role.as_str())
    }
}
