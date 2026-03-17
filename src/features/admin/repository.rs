use crate::error::AppError;
use crate::schema::{Shows};
use sqlx::PgPool;
use sea_query::{Query, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use serde_json::Value;
use sqlx::FromRow;
use anyhow::anyhow;

pub struct CreateShowParams {
    pub title: String,
    pub slug: String,
    pub cover_image_url: String,
    pub alternative_titles: Option<Value>,
    pub metadata: Option<Value>,
    pub synopsis: Option<String>,
    pub status: Option<String>,
}

#[derive(FromRow, Serialize)]
pub struct ShowsModel {
  id: uuid::Uuid,
  title: String,
  slug: String,
  cover_image_url: String,
  metadata: Option<Value>,
  alternative_titles: Option<Value>,
  synopsis: Option<String>,
  status: Option<String>,
}

pub async fn create_show(
  db: &PgPool,
  payload: CreateShowParams
) -> Result<ShowsModel, AppError> {
  let (sql, values) = Query::insert()
    .into_table(Shows::Table)
    .columns([Shows::Title, Shows::Slug, Shows::CoverImageUrl, Shows::Metadata, Shows::AlternativeTitles, Shows::Synopsis, Shows::Status])
    .values_panic([
      payload.title.into(),
      payload.slug.into(),
      payload.cover_image_url.into(),
      payload.metadata.map_or(sea_query::Value::Json(None).into(), |m| sea_query::Value::Json(Some(Box::new(m))).into()),
      payload.alternative_titles.map_or(sea_query::Value::Json(None).into(), |m| sea_query::Value::Json(Some(Box::new(m))).into()),
      payload.synopsis.into(),
      payload.status.into(),
    ])
    .returning(Query::returning().columns([Shows::Id, Shows::Title, Shows::Slug, Shows::CoverImageUrl, Shows::Metadata, Shows::AlternativeTitles, Shows::Synopsis, Shows::Status]))
    .build_sqlx(PostgresQueryBuilder);

  let show: ShowsModel = sqlx::query_as_with::<_, ShowsModel, _>(&sql, values)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Internal(anyhow!("Erro DB: {}", e)))?;

  Ok(show)
}
