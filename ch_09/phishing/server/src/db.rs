use chrono::Utc;
use common::api::model;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::{str::FromStr, time::Duration};

pub const DATABASE_URL: &str = "sqlite://db.sqlite";

pub async fn connect(database_url: &str) -> Result<SqlitePool, crate::Error> {
    let pool_timeout = Duration::from_secs(60);
    let pool_max_connections = 1;

    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(pool_timeout);

    let pool = SqlitePoolOptions::new()
        .max_connections(pool_max_connections)
        .idle_timeout(pool_timeout)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./db").run(&pool).await?;

    Ok(pool)
}

pub async fn insert(pool: &SqlitePool, credentials: &model::Login) -> Result<(), crate::Error> {
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO credentials (timestamp, email, password)
        VALUES (?, ?, ?)",
    )
    .bind(now)
    .bind(&credentials.email)
    .bind(&credentials.password)
    .execute(pool)
    .await?;

    Ok(())
}
