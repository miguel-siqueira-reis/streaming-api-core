mod handlers;
mod repository;

use axum::{routing::get, Router};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_catalog))
        .route("/{id}", get(handlers::get_catalog_item))
}
