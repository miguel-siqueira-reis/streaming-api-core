use axum::extract::State;
use axum::Json;
use sqlx::PgPool;
use serde::Deserialize;
use serde_json::Value;

use crate::error::AppError;
use super::repository::{self, ShowsModel, CreateShowParams};

#[derive(Deserialize)]
pub struct CreateShowRequest {
    title: String,
    slug: String,
    cover_image_url: String,
    metadata: Option<Value>,
}

impl From<CreateShowRequest> for CreateShowParams {
    fn from(req: CreateShowRequest) -> Self {
        Self {
            title: req.title,
            slug: req.slug,
            cover_image_url: req.cover_image_url,
            metadata: req.metadata,
            alternative_titles: None,
            synopsis: None,
            status: None,
        }
    }
}

pub async fn create_show(
  State(db): State<PgPool>,
  Json(payload): Json<CreateShowRequest>
) -> Result<Json<ShowsModel>, AppError> {

  let params: CreateShowParams = payload.into();

  let show = repository::create_show(&db, params).await?;

  Ok(Json(show))
}
