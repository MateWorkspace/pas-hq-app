use uuid::Uuid;

use crate::domain::models::{AccessClaims, AppContext};

/// Builds request-scoped context from transport-supplied actor and trace values.
///
/// The implementation generates a service-local execution ID and never derives it
/// from caller input. Claims are attached only when the caller already validated an
/// access token.
pub trait AppContextFactory: Send + Sync {
    fn create(
        &self,
        actor: Option<String>,
        trace_id: Option<Uuid>,
        claims: Option<AccessClaims>,
    ) -> AppContext;
}
