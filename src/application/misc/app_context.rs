use std::sync::Arc;

use crate::domain::{
    contracts::utility::app_context::AppContextFactory,
    models::{AccessClaims, AppContext, UsecaseError},
    usecases::{UsecaseFuture, misc::CreateAppContext},
};

/// Builds presentation-facing contexts over the injected factory.
pub struct AppContextCreator {
    factory: Arc<dyn AppContextFactory>,
}

impl AppContextCreator {
    pub fn new(factory: Arc<dyn AppContextFactory>) -> Self {
        Self { factory }
    }

    fn build(
        &self,
        actor: &str,
        trace_id: Option<uuid::Uuid>,
        claims: Option<AccessClaims>,
    ) -> Result<AppContext, UsecaseError> {
        Ok(self
            .factory
            .create(Some(actor.to_owned()), trace_id, claims))
    }
}

impl CreateAppContext for AppContextCreator {
    fn create<'a>(
        &'a self,
        actor: Option<String>,
        trace_id: Option<uuid::Uuid>,
        claims: Option<AccessClaims>,
    ) -> UsecaseFuture<'a, AppContext> {
        Box::pin(async move { Ok(self.factory.create(actor, trace_id, claims)) })
    }

    fn create_open<'a>(&'a self, trace_id: Option<uuid::Uuid>) -> UsecaseFuture<'a, AppContext> {
        Box::pin(async move { self.build("open", trace_id, None) })
    }

    fn create_system<'a>(&'a self, trace_id: Option<uuid::Uuid>) -> UsecaseFuture<'a, AppContext> {
        Box::pin(async move { self.build("system", trace_id, None) })
    }
}
