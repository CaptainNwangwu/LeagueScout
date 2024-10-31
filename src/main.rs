mod api;
mod config;
mod models;
mod services;

use crate::services::form_handler::{handle_player_search, serve_player_search_form};
use crate::services::page_handler::serve_search_results_page;
use actix_web::{web, App, HttpServer};
use config::setup::init_env;
use std::io::Result;

/*  Personal Note: We return std::io::Result because we are dealing with potential I/O errors.

Given that network operations, file handling, etc. can, and do often fail, we want to propagate
these errors to the Rust runtime so they can be managed accordingly (logging the error,
gracefully terminating the program, etc.)
*/
#[actix_web::main]
async fn main() -> Result<()> {
    // Reads from environment variables
    init_env();

    // Spin up web server :D
    println!("Starting web server...");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(serve_player_search_form)) // Route to serve the player search form
            .service(web::resource("/search-results").to(serve_search_results_page))
            .service(handle_player_search) // Route to handle the player search
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
