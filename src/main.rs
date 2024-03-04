// /* example from book

//     curl http://127.0.0.1:8000
//     bash: Hello World!
// */
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// // use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// async fn health_check() -> impl Responder {
// // async fn health_check(req: HttpRequest) -> impl Responder {// compiler warns about req not being used (could add _ to suppress warning or do as above
//     HttpResponse::Ok()
// }

// // async fn greet(req: HttpRequest) -> impl Responder {
// //     let name = req.match_info().get("name").unwrap_or("World");
// //     format!("Hello {}!", &name)
// // }

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             // .route("/", web::get().to(greet))
//             // .route("/{name}", web::get().to(greet))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// /*
//     example directly from actix-web documentation
//     curl http:/27.0.0.1:8080/321/ezequiel/index.html
//     bash: Hello ezequiel! id:321
// */
// // use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// // #[get("/{id}/{name}/index.html")]
// // async fn index(path: web::Path<(u32, String)>) -> impl Responder {
// //     let (id, name) = path.into_inner();
// //     format!("Hello {}! id:{}", name, id)
// // }
// // async fn health_check() -> impl Responder {
// //     HttpResponse::Ok()
// // }

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     HttpServer::new(|| {

// //         App::new()
// //             .service(index)
// //             .route("/health_check", web::get().to(health_check))
// //     })
// //         .bind("127.0.0.1:8000")?
// //         .run()
// //         .await
// // }

use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener: std::net::TcpListener = std::net::TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
    // run("127.0.0.1:8000")?.await
}
