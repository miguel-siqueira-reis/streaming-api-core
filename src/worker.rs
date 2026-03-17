use crate::state::AppState;
use crate::features::stream::repository;
use tokio::sync::{mpsc, Semaphore};
use tokio::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::path::Path;
use tracing::{info, error};

pub struct VideoCommand {
    pub id: uuid::Uuid,
    pub file_path: String,
}

pub async fn run(mut receiver: mpsc::Receiver<VideoCommand>, state: AppState) {
    let max_workers = state.config.max_concurrent_transcodes as usize;
    let semaphore = Arc::new(Semaphore::new(max_workers));

    info!("👷 [Worker] Subiu com limite de {} FFmpegs simultâneos.", max_workers);

    while let Some(command) = receiver.recv().await {

        let sem_clone = semaphore.clone();
        let state_clone = state.clone();

        tokio::spawn(async move {
            info!("📥 [Worker] Vídeo na fila: {}. Aguardando vaga...", command.file_path);

            let _permit = match sem_clone.acquire().await {
                Ok(p) => p,
                Err(_) => return,
            };

            info!("🎬 [Worker] Vaga liberada! Transcode HLS iniciado: {}", command.file_path);

            // Banco de Dados: Atualiza para "PROCESSING"
            let _ = repository::update_episode_status(&state_clone.db, command.id, "PROCESSING").await;

            let p = Path::new(&command.file_path);
            let file_stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("video_anonimo");

            let out_dir = format!("./storage/stream/{}", file_stem);
            let _ = tokio::fs::create_dir_all(&out_dir).await;

            let m3u8_path = format!("{}/playlist.m3u8", out_dir);

            let mut child = match Command::new(&state_clone.config.ffmpeg_path)
                .arg("-y")
                .arg("-i")
                .arg(&command.file_path)
                .arg("-vf")
                .arg("scale=-1:720")
                .arg("-c:v")
                .arg("libx264")
                .arg("-preset")
                .arg("veryfast")
                .arg("-g")
                .arg("60")
                .arg("-sc_threshold")
                .arg("0")
                .arg("-hls_time")
                .arg("2")
                .arg("-hls_list_size")
                .arg("0")
                .arg("-f")
                .arg("hls")
                .arg(&m3u8_path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(process) => process,
                Err(e) => {
                    error!("💥 [Worker] Falha ao invocar FFmpeg: {}", e);
                    // Banco de Dados: Atualiza para "ERROR" em falha crítica
                    let _ = repository::update_episode_status(&state_clone.db, command.id, "ERROR").await;
                    return;
                }
            };

            match child.wait().await {
                Ok(status) if status.success() => {
                    info!("✅ [Worker] Sucesso HLS! Playlist em: {}", m3u8_path);
                    // Banco de Dados: Atualiza para "READY" no sucesso!
                    let _ = repository::update_episode_status(&state_clone.db, command.id, "READY").await;
                }
                Ok(status) => {
                    error!("❌ [Worker] FFmpeg falhou com código de saída: {}", status);
                    let _ = repository::update_episode_status(&state_clone.db, command.id, "ERROR").await;
                }
                Err(e) => {
                    error!("🚨 [Worker] Erro inesperado ao aguardar: {}", e);
                    let _ = repository::update_episode_status(&state_clone.db, command.id, "ERROR").await;
                }
            }
        });
    }
}
