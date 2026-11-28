use super::router::stub_app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

/// Query parsing is the fragile part of the list endpoints: the extractor uses
/// `serde_urlencoded`, which cannot deserialize `flatten`. These cases fail loudly
/// if paging is ever moved back into a nested struct.
#[tokio::test]
async fn list_endpoints_accept_query_strings_with_and_without_paging() {
    let cases = [
        "/v1/products",
        "/v1/products?limit=5&offset=0",
        "/v1/products?brand_contains=app",
        "/v1/roles?limit=10",
        "/v1/roles?is_default=true",
        "/v1/permissions?limit=5",
        "/v1/users?limit=5&is_active=true",
        "/v1/locations?limit=10&kind=store",
        "/v1/stock/by-sku/1?limit=5",
        "/v1/stock/movements/by-sku/1?limit=5",
    ];

    for path in cases {
        let app = stub_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("authorization", "Bearer admin-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "{path} must parse its query string"
        );
    }
}
