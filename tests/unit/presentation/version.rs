use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use super::router::stub_app;

/// The version endpoint is public: it must answer without a bearer token or API
/// key, and expose only build metadata.
#[tokio::test]
async fn version_is_public_and_reports_build_metadata() {
    let app = stub_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/version")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["name"], "pas-hq-app");
    assert_eq!(json["version"], "0.1.0-test");
}

/// Guards the open/protected split: everything else still requires credentials.
#[tokio::test]
async fn protected_routes_still_require_credentials() {
    for path in ["/v1/products", "/v1/roles", "/v1/locations", "/v1/profile"] {
        let app = stub_app();

        let response = app
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "{path} must not be public"
        );
    }
}
