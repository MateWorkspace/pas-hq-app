use axum::http::HeaderMap;

use crate::presentation::http::dto::response::error::ApiError;

/// Read a header that must appear at most once, as non-blank ASCII text.
///
/// A missing header returns `Ok(None)`; every malformed shape is a client error so
/// the caller can reject it before any credential work happens.
pub(crate) fn single_header(headers: &HeaderMap, key: &str) -> Result<Option<String>, ApiError> {
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
