// src/routes/mod.rs

pub mod hello_user;
pub mod home;

// Define the logging function and make it public
pub fn logging(path: &str) {
    println!("{}", path);
}

// Export the route functions with unique names to avoid conflicts
pub use hello_user::hello_user as hello_user_handler; // Expose hello_user function
pub use home::home as home_handler;                   // Expose home function
