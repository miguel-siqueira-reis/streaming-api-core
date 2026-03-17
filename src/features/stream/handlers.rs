use axum::extract::State;
use tokio::sync::mpsc;
use axum::Json;
use serde_json::{json, Value};
use serde::Deserialize;
use crate::worker::VideoCommand;
use crate::error::AppError;
use crate::state::AppState;
use super::repository;

#[derive(Deserialize)]
pub struct TranscodeRequest {
    pub anime_id: uuid::Uuid,
    pub file_path: String,
    pub resolution: String,
}

pub async fn start_transcode(
    State(worker): State<mpsc::Sender<VideoCommand>>,
    State(state): State<AppState>,
    Json(payload): Json<TranscodeRequest>
) -> Result<Json<Value>, AppError> {

    // 1. Cria o episódio no banco como "PENDING"
    let episode_id = repository::create_episode(&state.db, payload.anime_id, &payload.file_path).await?;

    // 2. Manda pra fila de transcode passando o ID gerado!
    let command = VideoCommand {
        id: episode_id,
        file_path: payload.file_path,
    };

    worker.send(command)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro no canal: {}", e)))?;

    Ok(Json(json!({
        "status": "Transcode enfileirado",
        "episode_id": episode_id,
        "resolution_target": payload.resolution
    })))
}
