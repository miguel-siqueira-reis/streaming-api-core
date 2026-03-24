use serde::{Deserialize, Serialize};
use axum::{extract::State, Json, http::HeaderMap};

use crate::state::AppState;
use super::error::AuthError;
use super::repository::{self, UserModel, RegisterUserData};
use super::crypto;
use super::jwt;
use super::extractor::AuthenticatedUser;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
  State(app_state): State<AppState>,
  Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, AuthError> {

  let user: Option<UserModel> = repository::find_user_by_email(&app_state.db, &payload.email).await
        .map_err(AuthError::DatabaseError)?;

  let user = user.ok_or(AuthError::UserNotFound)?;

  let password_is_valid = crypto::verify_password(&payload.password, &user.password_hash)?;
  if !password_is_valid {
    return Err(AuthError::InvalidCredentials);
  }

  let token = jwt::create_token(user.id, user.role, app_state.config.jwt_secret.as_bytes())?;
  Ok(Json(LoginResponse { token }))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    name: String,
    username: String,
    email: String,
    password: String,
    password_confirm: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    id: String,
    token: String,
}

pub async fn register(
  State(app_state): State<AppState>,
  Json(payload): Json<RegisterRequest>
) -> Result<Json<RegisterResponse>, AuthError> {
    if payload.password != payload.password_confirm {
      return Err(AuthError::PasswordMismatch);
    }

    let has_user = repository::find_user_by_email(&app_state.db, &payload.email).await
          .map_err(AuthError::DatabaseError)?;
    if has_user.is_some() {
      return Err(AuthError::UserAlreadyExists);
    }

    let user_data = RegisterUserData {
      id: uuid::Uuid::new_v4(),
      name: payload.name,
      username: payload.username,
      email: payload.email,
      password_hash: crypto::hash_password(&payload.password)?,
      role: "USER".to_string(),
    };

    let token_id = user_data.id.clone();
    let token_role = user_data.role.clone();

    repository::create_user(&app_state.db, user_data).await
      .map_err(AuthError::DatabaseError)?;

    let token = jwt::create_token(token_id, token_role, app_state.config.jwt_secret.as_bytes())?;
    Ok(Json(RegisterResponse {
      id: token_id.to_string(),
      token,
    }))
}

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

pub async fn me(
  user: AuthenticatedUser,
) -> Result<Json<MeResponse>, AuthError> {
  // A magica do Extractor aconteceu!
  Ok(Json(MeResponse {
      id: user.0.id.to_string(),
      name: user.0.name,
      username: user.0.username,
      email: user.0.email,
      role: user.0.role,
  }))
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub token: String,
}

pub async fn refresh(
  State(app_state): State<AppState>,
  Json(payload): Json<RefreshRequest>
) -> Result<Json<LoginResponse>, AuthError> {
    let claims = jwt::verify_token(&payload.token, app_state.config.jwt_secret.as_bytes())?;

    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AuthError::InvalidCredentials)?;

    let new_token = jwt::create_token(user_id, claims.role, app_state.config.jwt_secret.as_bytes())?;

    Ok(Json(LoginResponse { token: new_token }))
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

pub async fn logout() -> Json<LogoutResponse> {

  Json(LogoutResponse {
      message: "Logout realizado com sucesso (limpe o token no cliente)".to_string(),
  })
}
