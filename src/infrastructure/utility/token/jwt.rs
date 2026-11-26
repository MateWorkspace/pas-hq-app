use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{Error as JwtError, ErrorKind},
};
use serde::{Deserialize, Serialize};

use crate::domain::{
    contracts::utility::token::Token,
    models::{AccessClaims, RefreshClaims, TokenError},
};

const ACCESS_TYPE: &str = "access";
const REFRESH_TYPE: &str = "refresh";

/// Access and refresh tokens signed with one shared HS256 secret.
///
/// Access and refresh tokens share the configured secret but are separated by a
/// token-type claim and independent lifetimes. The adapter emits no logs and reads
/// no live state: callers must recheck identity and grants before trusting refreshed
/// claims.
pub struct JwtToken {
    secret: Vec<u8>,
    access_duration: Duration,
    refresh_duration: Duration,
}

impl JwtToken {
    pub fn new(secret: String, access_duration: Duration, refresh_duration: Duration) -> Self {
        Self {
            secret: secret.into_bytes(),
            access_duration,
            refresh_duration,
        }
    }

    fn check_secret(&self) -> Result<(), TokenError> {
        if self.secret.is_empty() {
            Err(TokenError::Failed)
        } else {
            Ok(())
        }
    }

    fn check_identity(&self, user_id: i64) -> Result<(), TokenError> {
        if user_id <= 0 {
            Err(TokenError::BadArgs)
        } else {
            Ok(())
        }
    }

    fn checked_times(&self, duration: Duration) -> Result<(u64, u64, u64), TokenError> {
        let seconds = duration.as_secs();
        if seconds < 1 {
            return Err(TokenError::Failed);
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TokenError::Failed)?
            .as_secs();

        Ok((now, now, now + seconds))
    }

    fn validation(&self) -> Validation {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;
        validation.validate_nbf = true;
        validation
    }

    fn decode_claims<T: serde::de::DeserializeOwned>(
        &self,
        token: &str,
        validation: &Validation,
    ) -> Result<T, TokenError> {
        self.check_secret()?;

        decode::<T>(token, &DecodingKey::from_secret(&self.secret), validation)
            .map(|data| data.claims)
            .map_err(classify)
    }
}

impl Token for JwtToken {
    fn generate_access(&self, claims: &AccessClaims) -> Result<String, TokenError> {
        self.check_identity(claims.user_id)?;
        self.check_secret()?;
        let (iat, nbf, exp) = self.checked_times(self.access_duration)?;
        let payload = AccessPayload {
            user_id: claims.user_id,
            name: claims.name.clone(),
            role: claims.role.clone(),
            token_type: ACCESS_TYPE.to_owned(),
            sub: claims.user_id.to_string(),
            iat,
            nbf,
            exp,
        };

        sign(&payload, &self.secret)
    }

    fn validate_access(&self, token: &str) -> Result<AccessClaims, TokenError> {
        let payload = self.decode_claims::<AccessPayload>(token, &self.validation())?;
        if payload.token_type != ACCESS_TYPE
            || payload.user_id <= 0
            || payload.sub != payload.user_id.to_string()
        {
            return Err(TokenError::Invalid);
        }

        Ok(AccessClaims {
            user_id: payload.user_id,
            name: payload.name,
            role: payload.role,
        })
    }

    fn generate_refresh(&self, claims: &RefreshClaims) -> Result<String, TokenError> {
        self.check_identity(claims.user_id)?;
        self.check_secret()?;
        let (iat, nbf, exp) = self.checked_times(self.refresh_duration)?;
        let payload = RefreshPayload {
            user_id: claims.user_id,
            name: claims.name.clone(),
            token_type: REFRESH_TYPE.to_owned(),
            sub: claims.user_id.to_string(),
            iat,
            nbf,
            exp,
        };

        sign(&payload, &self.secret)
    }

    fn validate_refresh(&self, token: &str) -> Result<RefreshClaims, TokenError> {
        let payload = self.decode_claims::<RefreshPayload>(token, &self.validation())?;
        if payload.token_type != REFRESH_TYPE
            || payload.user_id <= 0
            || payload.sub != payload.user_id.to_string()
        {
            return Err(TokenError::Invalid);
        }

        Ok(RefreshClaims {
            user_id: payload.user_id,
            name: payload.name,
        })
    }
}

fn sign<T: Serialize>(payload: &T, secret: &[u8]) -> Result<String, TokenError> {
    encode(
        &Header::new(Algorithm::HS256),
        payload,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|_| TokenError::Failed)
}

fn classify(error: JwtError) -> TokenError {
    match error.kind() {
        ErrorKind::ExpiredSignature => TokenError::Expired,
        _ => TokenError::Invalid,
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AccessPayload {
    user_id: i64,
    name: String,
    role: String,
    token_type: String,
    sub: String,
    iat: u64,
    nbf: u64,
    exp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct RefreshPayload {
    user_id: i64,
    name: String,
    token_type: String,
    sub: String,
    iat: u64,
    nbf: u64,
    exp: u64,
}
