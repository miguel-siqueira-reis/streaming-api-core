use sea_query::{Expr, Query, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use crate::error::AppError;
use crate::schema::Anime;

#[derive(FromRow, Serialize)]
pub struct AnimeModel {
    pub id: uuid::Uuid,
    pub name: String,
}

pub async fn list_animes(db: &PgPool) -> Result<Vec<AnimeModel>, AppError> {
    let (sql, values) = Query::select()
        .column(Anime::Id)
        .column(Anime::Name)
        .from(Anime::Table)
        .and_where(Expr::col(Anime::Id).is_not_null())
        .build_sqlx(PostgresQueryBuilder);

    let animes: Vec<AnimeModel> = sqlx::query_as_with::<_, AnimeModel, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(animes)
}

pub async fn get_anime_by_id(db: &PgPool, id: uuid::Uuid) -> Result<Option<AnimeModel>, AppError> {
    let (sql, values) = Query::select()
        .column(Anime::Id)
        .column(Anime::Name)
        .from(Anime::Table)
        .and_where(Expr::col(Anime::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let anime = sqlx::query_as_with::<_, AnimeModel, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(anime)
}
