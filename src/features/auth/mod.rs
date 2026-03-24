pub mod handlers;
pub mod repository;
pub mod jwt;
pub mod error;
pub mod crypto;
pub mod extractor;

use axum::{Router, routing::post};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
      .route("/login", post(handlers::login))
      .route("/register", post(handlers::register))
      .route("/me", post(handlers::me))
      .route("/refresh", post(handlers::refresh))
      .route("/logout", post(handlers::logout))
}
