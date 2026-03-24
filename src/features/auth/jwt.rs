use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use super::error::AuthError;
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_token(user_id: Uuid, role: String, secret: &[u8]) -> Result<String, AuthError> {
    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role,
        exp,
        iat,
    };

    encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(secret)
    ).map_err(|_| AuthError::TokenCreationError)
}

pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default()
    )
    .map(|token_data| token_data.claims)
    .map_err(|_| AuthError::InvalidCredentials)
}
