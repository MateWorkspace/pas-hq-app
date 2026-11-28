use axum::http::HeaderMap;

use crate::presentation::http::dto::response::error::ApiError;

use super::header::single_header;

/// Standard authorization header.
pub(crate) const HEADER: &str = "authorization";

/// Extract the bearer token when the header is present.
///
/// A malformed `Authorization` header is rejected rather than ignored, so a client
/// is never silently downgraded to another credential.
pub(crate) fn token(headers: &HeaderMap) -> Result<Option<String>, ApiError> {
    let Some(value) = single_header(headers, HEADER)? else {
        return Ok(None);
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

    Ok(Some(token.to_owned()))
}
