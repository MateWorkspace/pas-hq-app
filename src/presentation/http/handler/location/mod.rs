mod location_errmap;
mod manage;

pub(crate) fn routes() -> axum::Router<std::sync::Arc<crate::presentation::http::Dependencies>> {
    manage::routes()
}
