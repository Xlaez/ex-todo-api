use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct  Claims{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: &str) -> Result<String, StatusCode>{
    let secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must have a value");
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(2);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claims  = Claims {iat, exp, email: email.to_string()};

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()),).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode>{
    let secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must have a value");
    let result: Result<TokenData<Claims>, StatusCode> = decode(&jwt_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}