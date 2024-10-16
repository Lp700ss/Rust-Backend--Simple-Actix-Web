// src/routes/mod.rs

pub mod hello_user;
pub mod home;
pub mod create_user;

// Define the logging function and make it public
pub fn logging(path: &str) {
    println!("{}", path);
}

// Export the route functions with unique names to avoid conflicts
pub use hello_user::hello_user as hello_user_handler; 
pub use home::home as home_handler;                   
pub use create_user::create_new_user as create_user_handler; 