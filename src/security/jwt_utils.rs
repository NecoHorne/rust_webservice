use std::env;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use jwt::header::HeaderType;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Default, Deserialize, Serialize)]
struct CustomClaim {
    jti: String,
    sub: String,
    iat: u64,
    exp: u64,
}

impl CustomClaim{
    pub fn new(jti: String, sub: String, iat: u64, exp: u64) -> Self{
        CustomClaim{
            jti,
            sub,
            iat,
            exp
        }
    }
}

///
/// Issue a new JWT
///
pub fn new_token(uid: String, email: String) -> Result<String, &'static str> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    };
    let claims = CustomClaim::new(uid, email, current_time_in_millis(), one_week_from_now());
    let unsigned_token = Token::new(header, claims);

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_e| "Invalid key")?;

    let signed_token = unsigned_token
        .sign_with_key(&key)
        .map_err(|_e| "Sign error")?;
    Ok(signed_token.into())
}

///
/// Issue a new JWT refresh token
///
pub fn new_refresh_token(uid: String, email: String) -> Result<String, &'static str> {

    let jwt_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    };
    let claims = CustomClaim::new(uid, email, current_time_in_millis(), ninety_days_from_now());
    let unsigned_token = Token::new(header, claims);

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_e| "Invalid key")?;

    let signed_token = unsigned_token
        .sign_with_key(&key)
        .map_err(|_e| "Sign error")?;
    Ok(signed_token.into())

}

///
/// Return the current time in milliseconds
///
fn current_time_in_millis() -> u64 {
    let start = Utc::now();
    start.timestamp_millis() as u64
}

///
/// Return the time stamp in millis for exactly one week from now
///
fn one_week_from_now() -> u64 {
    let start = Utc::now();
    start.checked_add_signed(Duration::weeks(1)).unwrap().timestamp_millis() as u64
}

///
/// Return the time stamp in millis for 90 days from now (used for refresh tokens)
///
fn ninety_days_from_now() -> u64 {
    let start = Utc::now();
    start.checked_add_signed(Duration::days(90)).unwrap().timestamp_millis() as u64
}

///
/// Verify the JWT
///
pub fn verify_token(token: &str) -> Result<String, &'static str>{
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_e| "Invalid key")?;

    let token: Token<Header, CustomClaim, _> =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Verification failed")?;

    let (_, claims) = token.into();

    if current_time_in_millis() < claims.exp {
        return Ok(claims.jti);
    }

    Err("Token Expired")
}

///
/// Verify the Refresh JWT
///
pub fn verify_refresh_token(token: &str) -> Result<String, &'static str>{
    let jwt_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_e| "Invalid key")?;

    let token: Token<Header, CustomClaim, _> =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Verification failed")?;

    let (_, claims) = token.into();
    if current_time_in_millis() < claims.exp {
        return Ok(claims.jti);
    }

    Err("Token Expired")
}
