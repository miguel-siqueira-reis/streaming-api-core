use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::time::Instant;

use crate::error::AppError;
use super::repository::{create_show, CreateShowParams};

#[derive(Deserialize, Debug)]
pub struct JikanResponse {
    pub data: Vec<JikanAnime>,
}

#[derive(Deserialize, Debug)]
pub struct JikanAnime {
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub images: JikanImages,
    pub synopsis: Option<String>,
    pub score: Option<f64>,
    pub year: Option<i32>,
    pub studios: Vec<JikanStudio>,
    pub genres: Vec<JikanGenre>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JikanImages {
    pub jpg: JikanJpg,
}

#[derive(Deserialize, Debug)]
pub struct JikanJpg {
    pub large_image_url: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JikanStudio {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct JikanGenre {
    pub name: String,
}

pub async fn seed_catalog(State(db): State<PgPool>) -> Result<Json<Value>, AppError> {
    let start_time = Instant::now();
    tracing::info!("Iniciando processo de Seeding da Jikan API...");

    let fetch_start = Instant::now();
    let mut animes_to_insert = Vec::new();

    for page in 1..=4 {
        let paged_url = format!("https://api.jikan.moe/v4/top/anime?limit=25&page={}", page);
        let raw_response = reqwest::get(&paged_url)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro ao buscar: {}", e)))?;

        let text_res = raw_response.text().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Erro ler char: {}", e)))?;

        let response: Result<JikanResponse, _> = serde_json::from_str(&text_res);

        match response {
            Ok(parsed) => animes_to_insert.extend(parsed.data),
            Err(e) => {
                tracing::error!("Pau no Parse do JSON na pagina {}: {} -> {}", page, e, text_res);
                return Err(AppError::Internal(anyhow::anyhow!("Parse Error na pag {}: {}", page, e)));
            }
        }

        // Evitar block por rate limit (3req/sec max)
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
    }

    let fetch_duration = fetch_start.elapsed();
    let total_items = animes_to_insert.len();

    let db_start = Instant::now();
    let mut tasks = Vec::new();

    for anime in animes_to_insert {
        let db_pool = db.clone();

        let task = tokio::spawn(async move {
            let slug = anime.title.to_lowercase()
                .replace(" ", "-")
                .replace(":", "")
                .replace("'", "")
                .replace("!", "")
                .replace(",", "");

            let studios: Vec<String> = anime.studios.into_iter().map(|s| s.name).collect();
            let genres: Vec<String> = anime.genres.into_iter().map(|g| g.name).collect();

            let cover = anime.images.jpg.large_image_url
                .unwrap_or_else(|| anime.images.jpg.image_url.clone().unwrap_or_default());

            let mut alt_titles = json!({});
            if let Some(en) = &anime.title_english {
                alt_titles["en"] = json!(en);
            }
            if let Some(ja) = &anime.title_japanese {
                alt_titles["ja"] = json!(ja);
            }

            let params = CreateShowParams {
                title: anime.title.clone(),
                slug,
                cover_image_url: cover.clone(),
                metadata: Some(json!({
                    "synopsis": anime.synopsis.clone().unwrap_or_default(),
                    "score": anime.score.unwrap_or(0.0),
                    "release_year": anime.year.unwrap_or(0),
                    "studios": studios,
                    "genres": genres,
                    "origin": "Jikan API Seeder",
                    "cover_image_url": cover
                })),
                alternative_titles: Some(alt_titles),
                synopsis: anime.synopsis,
                status: anime.status,
            };

            create_show(&db_pool, params).await
        });

        tasks.push(task);
    }

    let mut success_count = 0;
    let mut fail_count = 0;

    for task in tasks {
        match task.await {
            Ok(Ok(_)) => success_count += 1,
            _ => fail_count += 1,
        }
    }

    let db_duration = db_start.elapsed();
    let total_duration = start_time.elapsed();

    tracing::info!("Seeding Finalizado! {} sucessos, {} falhas.", success_count, fail_count);

    Ok(Json(json!({
        "message": "Seeding Finalizado com sucesso",
        "metrics": {
            "total_items_processed": total_items,
            "success_inserts": success_count,
            "failed_inserts": fail_count,
            "timings_ms": {
                "1_http_fetch_jikan": fetch_duration.as_millis(),
                "2_database_concurrent_inserts": db_duration.as_millis(),
                "3_total_execution": total_duration.as_millis()
            }
        }
    })))
}
