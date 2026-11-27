use uuid::Uuid;

use crate::domain::{
    contracts::utility::app_context::AppContextFactory,
    models::{AccessClaims, AppContext},
};

/// Builds AppContext with a fresh service-local execution ID.
#[derive(Clone, Copy, Debug, Default)]
pub struct UuidAppContextFactory;

impl UuidAppContextFactory {
    pub const fn new() -> Self {
        Self
    }
}

impl AppContextFactory for UuidAppContextFactory {
    fn create(
        &self,
        actor: Option<String>,
        trace_id: Option<Uuid>,
        claims: Option<AccessClaims>,
    ) -> AppContext {
        AppContext {
            actor,
            trace_id,
            execution_id: Some(Uuid::new_v4()),
            claims,
        }
    }
}
