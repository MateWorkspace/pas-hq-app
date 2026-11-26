use std::time::Duration;

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use pas_hq_app::{
    domain::{
        contracts::utility::token::Token,
        models::{AccessClaims, RefreshClaims, TokenError},
    },
    infrastructure::utility::token::JwtToken,
};
use serde_json::json;

const SECRET: &str = "test-secret";

fn token() -> JwtToken {
    JwtToken::new(
        SECRET.into(),
        Duration::from_secs(300),
        Duration::from_secs(3_600),
    )
}

fn access() -> AccessClaims {
    AccessClaims {
        user_id: 1,
        name: "Ada".into(),
        roles: vec!["admin".into()],
    }
}

fn refresh() -> RefreshClaims {
    RefreshClaims {
        user_id: 1,
        name: "Ada".into(),
    }
}

#[test]
fn access_tokens_round_trip_with_identity_and_roles() {
    let token = token();
    let raw = token.generate_access(&access()).unwrap();

    assert_eq!(token.validate_access(&raw), Ok(access()));
}

#[test]
fn refresh_tokens_round_trip() {
    let token = token();
    let raw = token.generate_refresh(&refresh()).unwrap();

    assert_eq!(token.validate_refresh(&raw), Ok(refresh()));
}

#[test]
fn token_types_are_not_interchangeable() {
    let token = token();
    let raw_access = token.generate_access(&access()).unwrap();
    let raw_refresh = token.generate_refresh(&refresh()).unwrap();

    assert_eq!(
        token.validate_refresh(&raw_access),
        Err(TokenError::Invalid)
    );
    assert_eq!(
        token.validate_access(&raw_refresh),
        Err(TokenError::Invalid)
    );
}

#[test]
fn a_different_secret_is_rejected() {
    let issuer = token();
    let raw = issuer.generate_access(&access()).unwrap();
    let verifier = JwtToken::new(
        "other-secret".into(),
        Duration::from_secs(300),
        Duration::from_secs(3_600),
    );

    assert_eq!(verifier.validate_access(&raw), Err(TokenError::Invalid));
}

#[test]
fn tampered_and_malformed_tokens_are_rejected() {
    let token = token();
    let mut tampered = token.generate_access(&access()).unwrap();
    tampered.push('x');

    assert_eq!(token.validate_access(&tampered), Err(TokenError::Invalid));
    assert_eq!(
        token.validate_access("not-a-token"),
        Err(TokenError::Invalid)
    );
}

#[test]
fn expired_tokens_are_classified_separately() {
    let token = token();
    let expired = encode(
        &Header::new(Algorithm::HS256),
        &json!({
            "user_id": 1,
            "name": "Ada",
            "roles": ["admin"],
            "token_type": "access",
            "sub": "1",
            "iat": 1_000_000,
            "nbf": 1_000_000,
            "exp": 1_000_001,
        }),
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap();

    assert_eq!(token.validate_access(&expired), Err(TokenError::Expired));
}

#[test]
fn invalid_identity_lifetimes_and_secrets_are_classified() {
    let token = token();
    let mut anonymous = access();
    anonymous.user_id = 0;

    assert_eq!(token.generate_access(&anonymous), Err(TokenError::BadArgs));

    let zero_lifetime = JwtToken::new(
        SECRET.into(),
        Duration::from_secs(0),
        Duration::from_secs(3_600),
    );
    assert_eq!(
        zero_lifetime.generate_access(&access()),
        Err(TokenError::Failed)
    );

    let empty_secret = JwtToken::new(
        String::new(),
        Duration::from_secs(300),
        Duration::from_secs(3_600),
    );
    assert_eq!(
        empty_secret.generate_access(&access()),
        Err(TokenError::Failed)
    );
    assert_eq!(
        empty_secret.validate_access("anything"),
        Err(TokenError::Failed)
    );
}
