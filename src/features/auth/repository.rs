use sqlx::{FromRow, PgPool};
use crate::schema::auth::User;
use sea_query::{Expr, Query, PostgresQueryBuilder};
use sea_query_binder::SqlxBinder;
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub async fn find_user_by_email(db: &PgPool, email: &str) -> Result<Option<UserModel>, sqlx::Error> {
    let (sql, values) = Query::select()
        .column(User::Id)
        .column(User::Name)
        .column(User::Username)
        .column(User::Email)
        .column(User::PasswordHash)
        .column(User::Role)
        .from(User::Table)
        .and_where(Expr::col(User::Email).eq(email))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, UserModel, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(user)
}

pub async fn find_user_by_id(db: &PgPool, user_id: Uuid) -> Result<Option<UserModel>, sqlx::Error> {
    let (sql, values) = Query::select()
        .column(User::Id)
        .column(User::Name)
        .column(User::Username)
        .column(User::Email)
        .column(User::PasswordHash)
        .column(User::Role)
        .from(User::Table)
        .and_where(Expr::col(User::Id).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, UserModel, _>(&sql, values)
        .fetch_optional(db)
        .await?;

    Ok(user)
}


pub struct RegisterUserData {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub async fn create_user(db: &PgPool, user: RegisterUserData) -> Result<(), sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(User::Table)
        .columns([User::Id, User::Name, User::Username, User::Email, User::PasswordHash, User::Role])
        .values_panic([
            user.id.into(),
            user.name.into(),
            user.username.into(),
            user.email.into(),
            user.password_hash.into(),
            user.role.into(),
        ])
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values)
        .execute(db)
        .await?;

    Ok(())
}
