use axum::extract::{State, Path};
use sqlx::PgPool;
use axum::Json;
use crate::error::AppError;
use super::repository::{self, AnimeModel};

pub async fn list_catalog(
    State(db): State<PgPool>
) -> Result<Json<Vec<AnimeModel>>, AppError> {
    let animes = repository::list_animes(&db).await?;

    Ok(Json(animes))
}

pub async fn get_catalog_item(
    State(db): State<PgPool>,
    Path(id): Path<uuid::Uuid>
) -> Result<Json<Option<AnimeModel>>, AppError> {
    let anime = repository::get_anime_by_id(&db, id).await?;

    Ok(Json(anime))
}
