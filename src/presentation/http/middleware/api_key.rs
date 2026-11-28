use axum::http::HeaderMap;

use crate::presentation::http::dto::response::error::ApiError;

use super::header::single_header;

/// Header carrying a long-lived API key.
pub(crate) const HEADER: &str = "x-api-key";

/// Extract the API key when the header is present.
pub(crate) fn key(headers: &HeaderMap) -> Result<Option<String>, ApiError> {
    single_header(headers, HEADER)
}
