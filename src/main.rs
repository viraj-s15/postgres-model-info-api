use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn testing(requesr: HttpRequest) -> impl Responder {
    let name = requesr.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(testing))
            .route("/{name}", web::get().to(testing))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
