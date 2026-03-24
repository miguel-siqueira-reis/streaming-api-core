use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::state::AppState;
use super::error::AuthError;
use super::jwt;
use super::repository::{self, UserModel};

#[derive(Debug)]
pub struct AuthenticatedUser(pub UserModel);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        let token_data = jwt::verify_token(bearer.token(), state.config.jwt_secret.as_bytes())?;

        let user_id = uuid::Uuid::parse_str(&token_data.sub)
            .map_err(|_| AuthError::InvalidCredentials)?;

        let user = repository::find_user_by_id(&state.db, user_id).await
            .map_err(AuthError::DatabaseError)?
            .ok_or(AuthError::UserNotFound)?;

        Ok(AuthenticatedUser(user))
    }
}
