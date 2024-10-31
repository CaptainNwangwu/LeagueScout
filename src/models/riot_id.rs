// src/models/riot_id.rs
use serde::Deserialize;

use crate::api::riot_api::fetch_riot_id;

#[derive(Debug, Deserialize)]
pub struct RiotID {
    /*  We use the `rename` from serde due to the API response's structure using the reverseCamelCase
    naming structure, which is against Rust standards, who stressed using snake_case.
    */
    puuid: String,
    #[serde(rename = "gameName")]
    game_name: String,
    #[serde(rename = "tagLine")]
    tag_line: String,
}

impl RiotID {
    fn new(puuid: String, game_name: String, tag_line: String) -> RiotID {
        return RiotID {
            puuid,
            game_name,
            tag_line,
        };
    }
    pub fn get_game_name(&self) -> &String {
        &self.game_name
    }
    pub fn get_tag_line(&self) -> &String {
        &self.tag_line
    }
    pub fn get_id(&self) -> &String {
        &self.puuid
    }
}

pub fn parse_player_name(player_name: &str) -> Option<(String, String)> {
    // We use .trim() to get rid any newline characters (If this fn was used on ostream)
    let name_slices: Vec<&str> = player_name.trim().split("#").collect();
    if name_slices.len() != 2 {
        return None;
    }

    let new_game_name = name_slices[0].to_string();
    let new_tag_line = name_slices[1].to_string();

    Some((new_game_name, new_tag_line))
}
