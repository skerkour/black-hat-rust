use actix_web::{web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/{id}/{name}").route(web::get().to(routes::index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
