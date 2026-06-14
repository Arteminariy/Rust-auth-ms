//! JWT signing/verification helpers.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::claims::{AccessClaim, RefreshClaim};
use crate::dto::token::TokenResponse;
use crate::models::users::UserModel;

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("jwt: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

pub fn encode_tokens(
    user: &UserModel,
    secret: &str,
    access_ttl_secs: u64,
    refresh_ttl_secs: u64,
    now_unix: u64,
) -> Result<TokenResponse, JwtError> {
    let now = now_unix;
    let access_claim = AccessClaim {
        id: user.id,
        name: user.name.clone(),
        role_id: user.role_id,
        exp: (now + access_ttl_secs) as usize,
    };
    let refresh_claim = RefreshClaim {
        sub: user.name.clone(),
        exp: (now + refresh_ttl_secs) as usize,
    };
    let key = EncodingKey::from_secret(secret.as_bytes());
    let access_token = encode(&Header::default(), &access_claim, &key)?;
    let refresh_token = encode(&Header::default(), &refresh_claim, &key)?;
    Ok(TokenResponse { access_token, refresh_token })
}

pub fn decode_access(token: &str, secret: &str) -> Result<AccessClaim, JwtError> {
    let data = decode::<AccessClaim>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub fn decode_refresh(token: &str, secret: &str) -> Result<RefreshClaim, JwtError> {
    let data = decode::<RefreshClaim>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

/// Generic JWT decode used by the auth service.
/// (Kept generic so future claim types can re-use it.)
pub fn decode_token<T: for<'de> serde::Deserialize<'de>>(token: &str, secret: &str) -> Result<T, JwtError> {
    let data = decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

/// Returns the current Unix epoch seconds. Wrapped so tests can mock it.
pub fn now_unix_secs() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

