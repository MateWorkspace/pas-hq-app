use std::sync::Arc;

use axum::{Router, middleware};

use super::{Dependencies, dto::response::error::ApiError, handler, middleware::context};

/// Assembles the HTTP surface.
///
/// Sign-in and refresh run under an open context; every other route requires a
/// bearer token and runs under the authenticated user's context.
pub fn router(dependencies: Dependencies) -> Router {
    let state = Arc::new(dependencies);

    let open_context = Arc::clone(&state.create_app_context);
    let auth_context = Arc::clone(&state.create_app_context);
    let auth_session = Arc::clone(&state.session);
    let auth_api_key = Arc::clone(&state.validate_api_key);

    let public = Router::new()
        .merge(handler::auth::routes())
        .layer(middleware::from_fn(move |request, next| {
            let create_app_context = Arc::clone(&open_context);
            async move { context::open(create_app_context, request, next).await }
        }));

    let protected = Router::new()
        .merge(handler::access::routes())
        .merge(handler::catalog::routes())
        .merge(handler::inventory::routes())
        .merge(handler::location::routes())
        .merge(handler::profile::routes())
        .layer(middleware::from_fn(move |request, next| {
            let create_app_context = Arc::clone(&auth_context);
            let session = Arc::clone(&auth_session);
            let api_keys = Arc::clone(&auth_api_key);
            async move {
                context::authenticated(create_app_context, session, api_keys, request, next).await
            }
        }));

    super::middleware::recovery(
        Router::new()
            .merge(public)
            .merge(protected)
            .fallback(not_found)
            .method_not_allowed_fallback(method_not_allowed)
            .with_state(state),
    )
}

async fn not_found() -> ApiError {
    ApiError::not_found("ROUTE_NOT_FOUND", "The requested route does not exist.")
}

async fn method_not_allowed() -> ApiError {
    ApiError::method_not_allowed("The HTTP method is not supported for this route.")
}
