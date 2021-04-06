use crate::api;
use actix_web::Responder;
use std::collections::HashMap;

pub async fn index() -> impl Responder {
    let mut data = HashMap::new();
    data.insert("hello", "world");
    api::Response::ok(data)
}
