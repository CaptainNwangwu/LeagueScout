use crate::models::riot_id::RiotID;
use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Result};
use std::path::PathBuf;
use tera::{Context, Tera};

/// Function for loading search results page
pub async fn serve_search_results_page() -> Result<NamedFile> {
    let path: PathBuf = ["src", "views", "search_results.html"].iter().collect();
    Ok(NamedFile::open(path)?)
}

/// Handler for displaying search results
// TODO: Figure out what this fn should return
pub async fn handle_search_results(
    riot_ids: Vec<RiotID>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("riot_ids", &riot_ids);

    let rendered = tera.render
}
