use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    UserNotFound,
    TokenCreationError,
    PasswordHashingError,
    PasswordMismatch,
    DatabaseError(sqlx::Error),
    Internal(anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Credenciais inválidas".to_string()),
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, "Usuário já existe".to_string()),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()),
            AuthError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno do servidor".to_string())
            }
             AuthError::TokenCreationError => {
                tracing::error!("Token creation error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar token".to_string())
            }
            AuthError::PasswordHashingError => {
                tracing::error!("Password hashing error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao processar senha".to_string())
            }
            AuthError::PasswordMismatch => (StatusCode::BAD_REQUEST, "As senhas não coincidem".to_string()),
            AuthError::DatabaseError(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno de banco de dados".to_string())
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
