use crate::db;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use common::api::model;
use sqlx::SqlitePool;

pub async fn login(
    Extension(pool): Extension<SqlitePool>,
    Json(credentials): Json<model::Login>,
) -> impl IntoResponse {
    let result = db::insert(&pool, &credentials).await;
    if result.is_ok() {
        (StatusCode::OK, Json(model::LoginResponse { ok: true }))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(model::LoginResponse { ok: false }),
        )
    }
}
