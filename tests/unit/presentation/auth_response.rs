use pas_hq_app::domain::usecases::auth::session::{SessionProfile, SessionTokens, SessionUser};
use pas_hq_app::presentation::http::dto::response::auth::{
    ProfileResponse, SessionResponse, SessionUserResponse,
};

fn user() -> SessionUser {
    SessionUser {
        id: 3,
        name: "Default User".into(),
        role_id: 3,
        role_name: "User".into(),
        role: "user".into(),
    }
}

/// A session response always carries the profile and permissions it was built
/// from, so a client needs exactly one call after sign-in or refresh.
#[test]
fn session_response_carries_tokens_and_profile() {
    let response = SessionResponse::from(SessionTokens {
        access_token: "access".into(),
        refresh_token: "refresh".into(),
        profile: SessionProfile {
            user: user(),
            permissions: vec!["product:list".into(), "product:get".into()],
        },
    });

    assert_eq!(response.access_token, "access");
    assert_eq!(response.refresh_token, "refresh");
    assert_eq!(response.user.id, 3);
    assert_eq!(response.user.role, "user");
    assert_eq!(response.permissions, ["product:list", "product:get"]);
}

/// The profile endpoint reports the same shape minus the tokens.
#[test]
fn profile_response_carries_identity_and_permissions() {
    let response = ProfileResponse::from(SessionProfile {
        user: user(),
        permissions: vec!["stock:list".into()],
    });

    assert_eq!(response.user.name, "Default User");
    assert_eq!(response.user.role_name, "User");
    assert_eq!(response.permissions, ["stock:list"]);
}

/// The token-facing view is derived from the same identity, so a token always
/// agrees with the profile it was issued for.
#[test]
fn session_user_projects_the_token_claims() {
    let claims = user().claims();

    assert_eq!(claims.user_id, 3);
    assert_eq!(claims.name, "Default User");
    assert_eq!(claims.role_id, 3);
    assert_eq!(claims.role_name, "User");
    assert_eq!(claims.role, "user");
}

#[test]
fn session_user_response_maps_every_field() {
    let response = SessionUserResponse::from(user());

    assert_eq!(response.id, 3);
    assert_eq!(response.name, "Default User");
    assert_eq!(response.role_id, 3);
    assert_eq!(response.role_name, "User");
    assert_eq!(response.role, "user");
}
