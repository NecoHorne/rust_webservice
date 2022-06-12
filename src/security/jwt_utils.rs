use std::env;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use jwt::header::HeaderType;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

///
/// Check a given string and determine if it is a jwt token
///
pub fn token_regex(token: &String) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*$").unwrap();
    re.is_match(token)
}

//=============================================TOKEN==============================================//

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
/// Verify the JWT and extract the claim from the token
///
fn extract_claim(token: &str) -> Result<CustomClaim, &'static str>{
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_e| "Invalid key")?;

    let token: Token<Header, CustomClaim, _> =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Verification failed")?;

    let (_, claims) = token.into();

    Ok(claims)
}

pub fn is_token_expired(token: &str) -> Result<bool, &'static str> {
    let claims = extract_claim(token).map_err(|e| e)?;
    Ok(current_time_in_millis() > claims.exp)
}

pub fn get_email_from_token(token: &str) -> Result<String, &'static str> {
    let claims = extract_claim(token).map_err(|e| e)?;
    Ok(claims.sub)
}

pub fn get_uid_from_token(token: &str) -> Result<String, &'static str> {
    let claims = extract_claim(token).map_err(|e| e)?;
    Ok(claims.jti)
}

///
/// Checks if token is valid
/// # Token :&str the jwt
/// # email :&str the email retrieved from the user in the db
///
pub fn is_token_valid(token: &str, email :&str) -> Result<bool, &'static str> {
    let expired = is_token_expired(token).map_err(|e| e)?;
    let email_claim = get_email_from_token(token).map_err(|e| e)?;
    return if !expired && email.eq(&email_claim) {
        Ok(true)
    } else {
        Ok(false)
    }
}

//========================================REFRESH TOKEN===========================================//

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
/// Verify the Refresh JWT and extract the claim from the token
///
fn extract_claim_refresh(token: &str) -> Result<CustomClaim, &'static str>{
    let jwt_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_e| "Invalid key")?;

    let token: Token<Header, CustomClaim, _> =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Verification failed")?;
    let (_, claims) = token.into();
    Ok(claims)
}

pub fn is_refresh_token_expired(token: &str) -> Result<bool, &'static str> {
    let claims = extract_claim_refresh(token).map_err(|e| e)?;
    Ok(current_time_in_millis() > claims.exp)
}

pub fn get_email_from_refresh(token: &str) -> Result<String, &'static str> {
    let claims = extract_claim_refresh(token).map_err(|e| e)?;
    Ok(claims.sub)
}

pub fn get_uid_from_refresh(token: &str) -> Result<String, &'static str> {
    let claims = extract_claim_refresh(token).map_err(|e| e)?;
    Ok(claims.jti)
}

///
/// Checks if refresh token is valid
/// # Token :&str the refresh jwt
/// # email :&str the email retrieved from the user in the db
///
pub fn is_refresh_token_valid(token: &str, email :&str) -> Result<bool, &'static str> {
    let expired = is_refresh_token_expired(token).map_err(|e| e)?;
    let email_claim = get_email_from_refresh(token).map_err(|e| e)?;
    return if !expired && email.eq(&email_claim) {
        Ok(true)
    } else {
        Ok(false)
    }
}

//==========================================TIMESTAMPS============================================//

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

//=========================================CUSTOM CLAIMS==========================================//

#[derive(Default, Deserialize, Serialize)]
struct CustomClaim {
    jti: String,
    sub: String,
    iat: u64,
    exp: u64,
}

impl CustomClaim {
    pub fn new(jti: String, sub: String, iat: u64, exp: u64) -> Self {
        Self { jti, sub, iat, exp }
    }
}
