use actix_web::{dev::Server, get, web, App, HttpServer, Responder};
use actix_web::web::Path;

#[get("/home")]
async fn home() -> impl Responder {
    let response: &str = "Welcome to the Actix Web Server";
    response
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let response: String = format!("Hello {} {}", params.0, params.1);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .service(home)
            .service(hello_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    println!("server is running in 127.0.0.1::8080");
    server.await
}
