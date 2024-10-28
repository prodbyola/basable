use std::str::from_utf8;

use crate::{utils::get_env, AppError};
use axum::http::{HeaderValue, StatusCode};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub(crate) struct User {
    pub id: String,
    pub is_guest: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: String::new(),
            is_guest: true,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Claims {
    sub: String,
    is_guest: bool,
    exp: usize,
}

#[derive(Serialize)]
pub(crate) struct JwtSession {
    pub token: String,
    pub exp: usize,
}

pub(crate) fn create_jwt(user: User) -> Result<JwtSession, AppError> {
    let exp_time = 7200;

    // Token expiration is two hours
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(exp_time.clone()))
        .expect("Invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        is_guest: user.is_guest,
        exp,
    };

    let header = Header::new(Algorithm::HS512);

    let secret = get_env("BASABLE_JWT_SECRET");
    let secret = secret.as_bytes();

    let token = encode(&header, &claims, &EncodingKey::from_secret(secret))
        .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(JwtSession {
        token,
        exp: exp_time as usize,
    })
}

pub(crate) fn decode_jwt(header_value: &HeaderValue) -> Result<User, AppError> {
    let token = extract_jwt(header_value)?;

    let secret = get_env("BASABLE_JWT_SECRET");
    let secret = secret.as_bytes();

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|e| AppError::HttpError(StatusCode::UNAUTHORIZED, e.to_string()))?;

    let Claims { sub, is_guest, .. } = decoded.claims;

    Ok(User { id: sub, is_guest })
}

fn extract_jwt(header_value: &HeaderValue) -> Result<String, AppError> {
    let mut jwt = Err(AppError::HttpError(
        StatusCode::UNAUTHORIZED,
        String::from("Invalid token!"),
    ));

    let bearer = format!("{} ", get_env("BASABLE_JWT_BEARER"));
    let bearer = bearer.as_str();

    if let Ok(v) = from_utf8(header_value.as_bytes()) {
        if v.starts_with(bearer) {
            let ext = v.trim_start_matches(bearer);
            jwt = Ok(ext.to_owned());
        }
    }

    jwt
}
