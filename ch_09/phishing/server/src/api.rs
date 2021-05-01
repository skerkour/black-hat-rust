use crate::db;
use common::api::model;
use sqlx::SqlitePool;
use std::convert::Infallible;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

pub fn with_db(
    db: Arc<SqlitePool>,
) -> impl Filter<Extract = (Arc<SqlitePool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn json_body() -> impl Filter<Extract = (model::Login,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn login(
    credentials: model::Login,
    db: Arc<SqlitePool>,
) -> Result<impl warp::Reply, Infallible> {
    let result = db::insert(&db, &credentials).await;
    match result {
        Ok(_) => {
            let res = warp::reply::json(&model::LoginResponse { ok: true });
            Ok(warp::reply::with_status(res, StatusCode::OK))
        }
        _ => {
            let res = warp::reply::json(&model::LoginResponse { ok: false });
            Ok(warp::reply::with_status(
                res,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
