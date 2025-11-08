use bcrypt::{hash, DEFAULT_COST};
use sqlx::types::chrono;

use crate::controllers::auth::SignUpRequest;

pub async fn has_with_email(db: &sqlx::postgres::PgPool, email: &str) -> Result<bool, sqlx::Error> {
    Ok(sqlx::query!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await?
        .is_some())
}

pub async fn create_user(
    db: &sqlx::postgres::PgPool,
    request: &SignUpRequest,
) -> Result<i64, Box<dyn std::error::Error>> {
    let hashed_password = hash(&request.password, DEFAULT_COST)?;

    let result = sqlx::query!(
        "INSERT INTO users (email, password, firstname, lastname) VALUES ($1, $2, $3, $4) RETURNING id",
        &request.email,
        hashed_password,
        &request.firstname,
        &request.lastname
    )
    .fetch_one(db)
    .await?;

    Ok(result.id)
}


pub struct User{
    pub id: i64,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub balance: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

pub async fn get_by_email(db: &sqlx::postgres::PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await
}
