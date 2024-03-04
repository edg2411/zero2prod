use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
email: String,
name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        // .bind("127.0.0.1:8000")?
        // .bind(address)?
        .listen(listener)?
        .run();
    // .await
    Ok(server)
}
