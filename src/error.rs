use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    Database(sqlx::Error),
    Internal(anyhow::Error),
    NotFound(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno de banco de dados".to_string())
            }
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno do servidor".to_string())
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            }
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
