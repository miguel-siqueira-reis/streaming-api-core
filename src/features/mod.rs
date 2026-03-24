pub mod catalog;
pub mod stream;
pub mod admin;
pub mod auth;

use axum::Router;
use tower_http::services::ServeDir;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/catalog", catalog::router())
        .nest("/stream", stream::router())
        .nest("/admin", admin::router())
        // Servindo a pasta dos m3u8 pra fora da internet (Direto do Kernel pro Request!)
        .nest_service("/hls", ServeDir::new("storage/stream"))
        .with_state(state)
}
