use sqlx::{MySqlPool, Row};
use crate::models::{User, CreateUser};

pub async fn create_connection_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    MySqlPool::connect(&database_url).await
}

pub async fn create_user(pool: &MySqlPool, user: CreateUser) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
        user.username,
        user.email,
        user.password_hash
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i32)
}

pub async fn get_user_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash FROM users WHERE email = ?",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &MySqlPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash FROM users WHERE id = ?",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}