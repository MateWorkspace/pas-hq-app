mod account;
mod profile_errmap;

pub(crate) fn routes() -> axum::Router<std::sync::Arc<crate::presentation::http::Dependencies>> {
    account::routes()
}
