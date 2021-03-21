use crate::db;
use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};
use common::api::model;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn login(
    pool: web::Data<Arc<SqlitePool>>,
    credentials: Json<model::Login>,
) -> impl Responder {
    let credentials = credentials.into_inner();
    let result = db::insert(&pool, &credentials).await;
    match result {
        Ok(_) => {
            let res = model::LoginResponse { ok: true };
            HttpResponse::Ok().json(res)
        }
        _ => HttpResponse::InternalServerError().body("Error"),
    }
}
