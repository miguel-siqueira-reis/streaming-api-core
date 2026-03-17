use sea_query::{Expr, Query, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use crate::error::AppError;
use anyhow::anyhow;
use crate::schema::Episodes;

pub async fn create_episode(db: &PgPool, show_id: uuid::Uuid, video_path: &str) -> Result<uuid::Uuid, AppError> {
    let (sql, values) = Query::insert()
        .into_table(Episodes::Table)
        .columns([Episodes::ShowId, Episodes::VideoPath, Episodes::TranscodeStatus, Episodes::Number])
        .values_panic([
            show_id.into(),
            video_path.into(),
            "PENDING".into(),
            1.into() // Dummy number 1 mapping
        ])
        .returning_col(Episodes::Id)
        .build_sqlx(PostgresQueryBuilder);

    // No Postgres com o returning_col habilitado pela query do SeaQuery, fetch_one nos dá o UUID limpo
    let row: (uuid::Uuid,) = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(anyhow!("Erro DB: {}", e)))?;

    Ok(row.0)
}

pub async fn update_episode_status(db: &PgPool, id: uuid::Uuid, status: &str) -> Result<(), AppError> {
    let (sql, values) = Query::update()
        .table(Episodes::Table)
        .values([(Episodes::TranscodeStatus, status.into())])
        .and_where(Expr::col(Episodes::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(anyhow!("Erro DB: {}", e)))?;

    Ok(())
}
