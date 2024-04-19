use std::str::from_utf8;

use axum::http::{HeaderValue, StatusCode};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::{config::Config, AppError};

// JWT_SECRET should be defined by the installer and saved in env variables.
// You can generate one at https://djecrety.ir
const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"n!d5-s4ab_mp^a=w)p83vphpbm%y2s7vc!re481*ycw&szsyff";

#[derive(Clone)]
pub(crate) struct User {
    pub id: String,
    pub is_logged: bool,
}

impl User {
    pub(crate) fn validate(&self) -> bool {
        false
    }

    pub(crate) fn logout(&self) {
        // TODO: Close connection
    }

    /// Saves this `Config` for user and create new connection using the `Config`.
    pub(crate) fn save_new_config(&self, config: &Config) {}
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize)]
pub(crate) struct JwtSession {
    pub token: String,
    pub exp: usize,
}

pub(crate) fn create_jwt(user_id: &str) -> Result<JwtSession, AppError> {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(120))
        .expect("Invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.clone(),
    };

    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| AppError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(JwtSession { token, exp })
}

pub(crate) fn decode_jwt(header_value: &HeaderValue) -> Result<String, AppError> {
    let token = extract_jwt(header_value)?;
    let decoded = decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(JWT_SECRET), 
        &Validation::new(Algorithm::HS512)
    ).map_err(|e| AppError(StatusCode::UNAUTHORIZED, e.to_string()))?;

    Ok(decoded.claims.sub)
}

fn extract_jwt(header_value: &HeaderValue) -> Result<String, AppError> {
    let mut jwt = Err(AppError(
        StatusCode::UNAUTHORIZED,
        String::from("Invalid token!"),
    ));

    if let Ok(v) = from_utf8(header_value.as_bytes()) {
        if v.starts_with(BEARER) {
            let ext = v.trim_start_matches(BEARER);
            jwt = Ok(ext.to_owned());
        }
    }

    jwt
}
