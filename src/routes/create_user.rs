// src/routes/create_user.rs

use actix_web::{http::StatusCode, post, web::Json, Responder}; // Import necessary types
use serde::{Deserialize, Serialize}; // Import both Deserialize and Serialize

use crate::routes::logging;  // Import the logging function
use crate::routes::hello_user::User; // Import User struct from hello_user.rs

#[derive(Deserialize, Serialize)] // Add Serialize here
pub struct CreateUserResponse { // Make this struct public
    id: u32,
    user: User,
}

// Ensure this function is public
#[post("/user/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    let route: String = format!("POST: /user/create");
    logging(&route);
    
    (
        Json(CreateUserResponse {
            id: 1, // Replace with actual logic if needed
            user: user.into_inner(), // Use into_inner to extract the User
        }),
        StatusCode::CREATED,
    )
}
