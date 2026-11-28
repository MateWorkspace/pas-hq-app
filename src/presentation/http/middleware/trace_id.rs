use axum::http::HeaderMap;
use uuid::Uuid;

use crate::presentation::http::dto::response::error::ApiError;

use super::header::single_header;

/// Header carrying the caller's correlation identifier.
pub(crate) const HEADER: &str = "x-trace-id";

/// Parse the optional trace identifier.
///
/// Absent is valid; a present value must be a non-zero UUID so a client cannot
/// smuggle a placeholder into logs.
pub(crate) fn from_headers(headers: &HeaderMap) -> Result<Option<Uuid>, ApiError> {
    let Some(value) = single_header(headers, HEADER)? else {
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
