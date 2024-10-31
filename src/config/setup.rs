// src/config/setup.rs
use dotenv::dotenv;
use std::env;

pub fn init_env() {
    dotenv().ok();
}

pub fn get_riot_api_key() -> String {
    env::var("RIOT_API_KEY").expect("RIOT_API_KEY must be set.")
}
