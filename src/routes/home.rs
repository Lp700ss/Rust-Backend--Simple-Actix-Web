// src/routes/home.rs
use actix_web::{get, Responder};

use crate::routes::logging;

#[get("/home")]
pub async fn home() -> impl Responder {

    let route: String = format!("GET: /home");
    logging(&route);
    let response: &str = "Welcome to the Actix Web Server";
    response
}
