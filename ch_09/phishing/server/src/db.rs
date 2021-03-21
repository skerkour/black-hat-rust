use chrono::Utc;
use common::api::model;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, SqlitePool,
};
use std::str::FromStr;

pub const DATABASE_URL: &str = "sqlite://db.sqlite";

pub async fn connect(database_url: &str) -> Result<SqlitePool, crate::Error> {
    let pool = SqlitePoolOptions::new().connect(database_url).await?;
    Ok(pool)
}

pub async fn init(database_url: &str) -> Result<(), crate::Error> {
    let mut connection = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .connect()
        .await?;
    sqlx::migrate!("./db").run(&mut connection).await?;

    Ok(())
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
