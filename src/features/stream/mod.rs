pub mod handlers;
pub mod repository;

use axum::{routing::post, Router};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/transcode", post(handlers::start_transcode))
}
