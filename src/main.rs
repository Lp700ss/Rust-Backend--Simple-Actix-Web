// src/main.rs

mod routes; // Declare the routes module

use actix_web::{dev::Server, App, HttpServer};
use crate::routes::{hello_user_handler, home_handler}; // Use renamed imports

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .service(home_handler)        // Ensure home is exposed
            .service(hello_user_handler)  // Ensure hello_user is exposed
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    
    println!("Server is running at http://127.0.0.1:8080");
    server.await
}
