use actix_web::http::StatusCode;
use actix_web::{dev::Server, get, web, App, HttpServer, Responder};
use actix_web::web::{Json, Path};
use serde::Serialize;

#[get("/home")]
async fn home() -> impl Responder {
    let response: &str = "Welcome to the Actix Web Server";
    response
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let response: User = User::new( params.0.clone(), params.1.clone());
    (Json(response), StatusCode::OK)
}


#[derive(Serialize)]
struct User {
    first_name: String,
    last_name: String
}

impl User {
    fn new(firstname: String, lastname: String) -> Self {
        Self {
            first_name: firstname,
            last_name: lastname,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server: Server = HttpServer::new(|| 
        App::new()
            .service(home)
            .service(hello_user)
    )
    .bind(("127.0.0.1", 8080))?
    .run();
    println!("server is running in 127.0.0.1::8080");
    server.await
}
