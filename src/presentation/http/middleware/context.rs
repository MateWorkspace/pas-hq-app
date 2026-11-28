use axum::{
    extract::{FromRequestParts, Request},
    http::{HeaderMap, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        models::{AppContext, UsecaseError},
        usecases::{auth::Session, misc::CreateAppContext},
    },
    presentation::http::dto::response::error::ApiError,
};

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
    let trace_id = match trace_id(request.headers()) {
        Ok(trace_id) => trace_id,
        Err(error) => return error.into_response(),
    };

    match create_app_context.create_open(trace_id).await {
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

/// Validates the bearer token and builds a context carrying the user's claims.
pub(crate) async fn authenticated(
    create_app_context: Arc<dyn CreateAppContext>,
    session: Arc<dyn Session>,
    mut request: Request,
    next: Next,
) -> Response {
    let trace_id = match trace_id(request.headers()) {
        Ok(trace_id) => trace_id,
        Err(error) => return error.into_response(),
    };

    let token = match bearer_token(request.headers()) {
        Ok(token) => token,
        Err(error) => return error.into_response(),
    };

    let system = match create_app_context.create_system(trace_id).await {
        Ok(context) => context,
        Err(error) => {
            return ApiError::internal(
                "REQUEST_CONTEXT_FAILED",
                format!("{CREATE_CONTEXT_FAILED} ({})", error.code()),
            )
            .into_response();
        }
    };

    let claims = match session.validate_access(&system, token.as_str()).await {
        Ok(claims) => claims,
        Err(UsecaseError::Token(_)) => {
            return ApiError::unauthorized(
                "ACCESS_TOKEN_INVALID",
                "Your session has expired or is not valid. Please sign in again.",
            )
            .into_response();
        }
        Err(error) => {
            return ApiError::internal(
                "ACCESS_VALIDATION_FAILED",
                format!("Your session could not be verified. ({})", error.code()),
            )
            .into_response();
        }
    };

    let actor = Some(claims.name.clone());
    match create_app_context
        .create(actor, trace_id, Some(claims))
        .await
    {
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

fn trace_id(headers: &HeaderMap) -> Result<Option<Uuid>, ApiError> {
    let Some(value) = single_header(headers, "x-trace-id")? else {
        return Ok(None);
    };
    let trace_id = Uuid::parse_str(&value).map_err(|_| {
        ApiError::bad_request("TRACE_ID_INVALID", "The x-trace-id header must be a UUID.")
    })?;
    if trace_id.is_nil() {
        return Err(ApiError::bad_request(
            "TRACE_ID_INVALID",
            "The x-trace-id header must not be the zero UUID.",
        ));
    }
    Ok(Some(trace_id))
}

fn bearer_token(headers: &HeaderMap) -> Result<String, ApiError> {
    let Some(value) = single_header(headers, "authorization")? else {
        return Err(ApiError::unauthorized(
            "AUTHORIZATION_REQUIRED",
            "A bearer token is required to access this resource.",
        ));
    };

    let mut parts = value.split_whitespace();
    let (Some(scheme), Some(token), None) = (parts.next(), parts.next(), parts.next()) else {
        return Err(ApiError::unauthorized(
            "AUTHORIZATION_INVALID",
            "The Authorization header must use the form \"Bearer <token>\".",
        ));
    };

    if !scheme.eq_ignore_ascii_case("bearer") {
        return Err(ApiError::unauthorized(
            "AUTHORIZATION_INVALID",
            "The Authorization header must use the Bearer scheme.",
        ));
    }

    Ok(token.to_owned())
}

fn single_header(headers: &HeaderMap, key: &str) -> Result<Option<String>, ApiError> {
    let mut values = headers.get_all(key).iter();
    let Some(value) = values.next() else {
        return Ok(None);
    };
    if values.next().is_some() {
        return Err(ApiError::bad_request(
            "REQUEST_HEADER_DUPLICATED",
            format!("The {key} header must appear exactly once."),
        ));
    }
    let value = value.to_str().map_err(|_| {
        ApiError::bad_request(
            "REQUEST_HEADER_INVALID",
            format!("The {key} header must be plain ASCII text."),
        )
    })?;
    if value.trim().is_empty() {
        return Err(ApiError::bad_request(
            "REQUEST_HEADER_INVALID",
            format!("The {key} header must not be empty."),
        ));
    }
    Ok(Some(value.to_owned()))
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
