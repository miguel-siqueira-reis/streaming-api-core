use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::worker::VideoCommand;
use std::env;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub worker_commander: mpsc::Sender<VideoCommand>,
    pub config: Arc<AppConfig>,
}

#[derive(Clone)]
pub struct AppConfig {
    pub storage_path: std::path::PathBuf,
    pub ffmpeg_path: std::path::PathBuf,
    pub max_concurrent_transcodes: usize,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            storage_path: env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage/raw".to_string()).into(),
            ffmpeg_path: "ffmpeg".into(), // Poderia vir do env também
            max_concurrent_transcodes: 4, // Poderia vir do env
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env map"),
        }
    }
}

impl AppState {
    pub fn new(db: PgPool, worker_commander: mpsc::Sender<VideoCommand>, config: AppConfig) -> Self {
        Self {
            db,
            worker_commander,
            config: Arc::new(config),
        }
    }
}
