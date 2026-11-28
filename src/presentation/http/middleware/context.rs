use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    domain::{
        models::{AppContext, UsecaseError},
        usecases::{
            auth::{ApiKeyAuth, Session},
            misc::CreateAppContext,
        },
    },
    presentation::http::dto::response::error::ApiError,
};

use super::{api_key, bearer, trace_id};

/// The authenticated request context extracted by handlers.
#[derive(Clone)]
pub struct RequestContext(pub AppContext);

const CREATE_CONTEXT_FAILED: &str =
    "The request context could not be prepared. Please try again later.";

/// Builds an open context for unauthenticated routes such as sign-in.
pub(crate) async fn open(
    create_app_context: Arc<dyn CreateAppContext>,
    mut request: Request,
    next: Next,
) -> Response {
    let trace = match trace_id::from_headers(request.headers()) {
        Ok(trace) => trace,
        Err(error) => return error.into_response(),
    };

    match create_app_context.create_open(trace).await {
        Ok(context) => {
            request.extensions_mut().insert(context);
            next.run(request).await
        }
        Err(error) => ApiError::internal(
            "REQUEST_CONTEXT_FAILED",
            format!("{CREATE_CONTEXT_FAILED} ({})", error.code()),
        )
        .into_response(),
    }
}

/// Resolves the caller's credentials and builds an authenticated context.
///
/// A bearer token takes precedence over an API key when both are present. The
/// claims are resolved under a system context, because there is no identity until
/// validation succeeds.
pub(crate) async fn authenticated(
    create_app_context: Arc<dyn CreateAppContext>,
    session: Arc<dyn Session>,
    api_keys: Arc<dyn ApiKeyAuth>,
    mut request: Request,
    next: Next,
) -> Response {
    let trace = match trace_id::from_headers(request.headers()) {
        Ok(trace) => trace,
        Err(error) => return error.into_response(),
    };

    let bearer_token = match bearer::token(request.headers()) {
        Ok(token) => token,
        Err(error) => return error.into_response(),
    };

    let api_key_value = match api_key::key(request.headers()) {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };

    if bearer_token.is_none() && api_key_value.is_none() {
        return ApiError::unauthorized(
            "AUTHORIZATION_REQUIRED",
            "A bearer token or API key is required to access this resource.",
        )
        .into_response();
    }

    let system = match create_app_context.create_system(trace).await {
        Ok(context) => context,
        Err(error) => {
            return ApiError::internal(
                "REQUEST_CONTEXT_FAILED",
                format!("{CREATE_CONTEXT_FAILED} ({})", error.code()),
            )
            .into_response();
        }
    };

    let claims = if let Some(token) = bearer_token {
        match session.validate_access(&system, token.as_str()).await {
            Ok(claims) => claims,
            Err(error) => return access_error(error),
        }
    } else {
        let value = api_key_value.unwrap_or_default();
        match api_keys.validate_api_key(&system, value.as_str()).await {
            Ok(claims) => claims,
            Err(error) => return api_key_error(error),
        }
    };

    match create_app_context.create_authenticated(claims, trace).await {
        Ok(context) => {
            request.extensions_mut().insert(context);
            next.run(request).await
        }
        Err(error) => ApiError::internal(
            "REQUEST_CONTEXT_FAILED",
            format!("{CREATE_CONTEXT_FAILED} ({})", error.code()),
        )
        .into_response(),
    }
}

fn access_error(error: UsecaseError) -> Response {
    match error {
        UsecaseError::Token(_) => ApiError::unauthorized(
            "ACCESS_TOKEN_INVALID",
            "Your session has expired or is not valid. Please sign in again.",
        ),
        other => ApiError::internal(
            "ACCESS_VALIDATION_FAILED",
            format!("Your session could not be verified. ({})", other.code()),
        ),
    }
    .into_response()
}

fn api_key_error(error: UsecaseError) -> Response {
    match error {
        UsecaseError::Unauthorized => ApiError::unauthorized(
            "API_KEY_INVALID",
            "The API key is invalid, disabled, or its owner may not authenticate.",
        ),
        other => ApiError::internal(
            "API_KEY_VALIDATION_FAILED",
            format!("Your API key could not be verified. ({})", other.code()),
        ),
    }
    .into_response()
}

impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AppContext>()
            .cloned()
            .map(Self)
            .ok_or_else(|| {
                ApiError::internal(
                    "REQUEST_CONTEXT_MISSING",
                    "The request context is missing from this route.",
                )
            })
    }
}
