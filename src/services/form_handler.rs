use crate::api::riot_api::fetch_riot_id_by_name;
use crate::api::riot_api::FetchRiotIdError;
use actix_files::NamedFile;
use actix_web::{post, web, HttpResponse, Result};
use serde::Deserialize;
use std::path::PathBuf;

// Struct for deserializing form data
#[derive(Deserialize)]
struct PlayerForm {
    player_name: String,
}

/// Handler for loading player search form
pub async fn serve_player_search_form() -> Result<NamedFile> {
    let path: PathBuf = ["src", "views", "player_search_form.html"].iter().collect();
    Ok(NamedFile::open(path)?)
}

// Handle form submission
#[post("/submit_player")]
pub async fn handle_player_search(form: web::Form<PlayerForm>) -> HttpResponse {
    let player_name = &form.player_name;

    // Fetch Riot ID struct from riot API
    match fetch_riot_id_by_name(player_name).await {
        Ok(riot_id) => HttpResponse::Ok().body(format!("Fetched RiotID: {:#?}", riot_id)),
        Err(e) => match e {
            FetchRiotIdError::ParseError(parse_msg) => {
                // Return 400 BAD_REQUEST for parsing error
                HttpResponse::BadRequest().body(format!("{}", parse_msg))
            }
            FetchRiotIdError::RequestError(req_err) => match req_err.status() {
                Some(reqwest::StatusCode::NOT_FOUND) => {
                    HttpResponse::NotFound().body("Player not found")
                }
                Some(reqwest::StatusCode::FORBIDDEN) => HttpResponse::Forbidden()
                    .body("You lack the permission to access this resource"),
                _ => HttpResponse::BadGateway().body(format!("API Request Error: {}", req_err)),
            },
        },
    }
}
