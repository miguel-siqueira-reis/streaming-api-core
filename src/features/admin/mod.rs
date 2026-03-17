mod handlers;
mod repository;
mod seeder;

use axum::{routing::post, Router};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/shows", post(handlers::create_show))
    .route("/seed", post(seeder::seed_catalog))
}
