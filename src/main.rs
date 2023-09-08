use crate::ml_models::Model;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
mod ml_models;

async fn testing(requesr: HttpRequest) -> impl Responder {
    let name = requesr.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(ml_models::init_routes)
            .route("/", web::get().to(testing))
            .route("/{name}", web::get().to(testing))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
