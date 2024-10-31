// src/api/riot_api.rs

use std::error::Error;

use crate::config::setup::get_riot_api_key;
use crate::models::riot_id::{parse_player_name, RiotID};
use std::fmt;
use urlencoding::decode;

/// Custom error type for handling RiotID fetches.
///
/// A ParseError represents a failure to parse a player_name provided into the
/// required format gamename#tagline (e.g Hooked on Flays#2949).
///
/// A RequestError represents a failure to to get a successful response from the RiotAPI
/// server for a player_name fetch request.
// ? Consider adding the ability to handle specifically NOT_FOUND response for non-existent players
#[derive(Debug)]
pub enum FetchRiotIdError {
    ParseError(String),           // Error for invalid name format
    RequestError(reqwest::Error), // Error for failed API requests
}

impl fmt::Display for FetchRiotIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchRiotIdError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FetchRiotIdError::RequestError(err) => write!(f, "Request Error: {}", err),
        }
    }
}

impl Error for FetchRiotIdError {} // Allow FetchRiotIdError to be used as a dyn Error

/*
Uses &str instead of &String for reducing memory overhead since these variables
are intended to be read only. */

/// Sends a GET request to the Riot API to retrieve a JSON containing
/// the puuid.
pub async fn fetch_riot_id(game_name: &str, tag_line: &str) -> Result<RiotID, reqwest::Error> {
    let api_key = get_riot_api_key();

    let url =
        format!(
    "https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
    game_name.to_string(), tag_line.to_string(), api_key
);

    println!("\nSending request to Riot API...");
    let response = reqwest::get(&url).await?;

    let response = response.error_for_status()?;
    /* Note: Handling HTTP responses with reqwest is weird.

    Some different approaches that I've tried:

    1)  Using if, else to where the if is the scenairo in which the request is successful (2xx) response
        using status().is_success() and the else is for anything else.

    2)  Using a match statement for status codes.

    The ladder seems to be more efficient than the former, but it takes a bit more mental effort to type
    out, since you need to figure out how to utilize response.error_for_status(), specifically the Err variant
    (since it returns a result type), which is a idiomatically rust-specific complexity to manage.
    */

    // Ok, so it turns out the simplest way to do this is to just propogate the error up to the caller.
    let player_data: RiotID = response.json().await?;
    Ok(player_data) // Return player data
}

pub async fn fetch_riot_id_by_puuid(puuid: &str) -> Result<RiotID, reqwest::Error> {
    let api_key = get_riot_api_key();

    let url = format!(
        "https://americas.api.riotgames.com/riot/account/v1/accounts/by-puuid/{}?api_key={}",
        puuid, api_key
    );

    let response = reqwest::get(&url).await?;

    let response = response.error_for_status()?;

    let player_data: RiotID = response.json().await?;
    Ok(player_data)
}

/// Generates a vector of RiotIDs based on the data fetched from the RiotAPI
/// after having parsed the name of the players.
///
/// Returns a tuple containing the vector of RiotIDs that succeeded and a vector
/// of tuples that contain the failed names, as well as the error message for why they
/// failed.
///
/// ! Consider refactoring due to potential space overhead of excessive use of <String> vs &str.
pub async fn fetch_team_riot_id(player_list: &Vec<String>) -> (Vec<RiotID>, Vec<(String, String)>) {
    let mut riot_id_list = Vec::new();
    let mut failed_names = Vec::new();

    for player_name in player_list {
        match parse_player_name(player_name) {
            Some((game_name, tag_line)) => {
                match fetch_riot_id(game_name.as_str(), tag_line.as_str()).await {
                    Ok(riot_id) => riot_id_list.push(riot_id), // Add to success list
                    Err(err) => failed_names.push((player_name.to_string(), err.to_string())),
                }
            }
            // If the parsing fails, add it to the failed list too.
            None => {
                failed_names.push((player_name.to_string(), "Invalid name format".to_string()));
            }
        }
    }
    (riot_id_list, failed_names)
}

/*
TODO: Document this function pls.
TODO: Create a custom error type to handle the distinct errors that occur in this context. (Parsing and reqwest Errors)
*/
pub async fn fetch_riot_id_by_name(player_name: &str) -> Result<RiotID, FetchRiotIdError> {
    if let Some((parsed_game_name, parsed_tag_line)) = parse_player_name(player_name) {
        // Attempt to fetch the RiotID
        match fetch_riot_id(&parsed_game_name.as_str(), &parsed_tag_line.as_str()).await {
            Ok(riot_id) => Ok(riot_id),
            // Wrap request error in FetchRiotIdError
            Err(err) => Err(FetchRiotIdError::RequestError(err)),
        }
    } else {
        // Return parsing error if name parse fails
        Err(FetchRiotIdError::ParseError(format!(
            "Invalid player name format: {}. Expected format `game_name#tag_line`",
            player_name
        )))
    }
}

/*
TODO: - Implement small web app for sending form to request top 10 champions for player and for team OPGG.
 */

pub fn decode_multi(multi: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let multi_parts: Vec<&str> = multi.split("=").collect();
    let encoded_playerlist = multi_parts.get(1).ok_or("No players found in the URL")?;

    // Decocde the URL encoded multi
    let decoded_playerlist = decode(encoded_playerlist)?;

    /*  The URLencoding only  ends up escaping the `%` character, leaving the players separated by commas,
    so we must split my crowds. */
    let players: Vec<String> = decoded_playerlist
        .split(",")
        /* We convert to strings so that we don't tie the lifetime of the vector's values to the lifetime of the
        `multi` string. Since split returns &str, if we didn't convert to strings, we cannot return the `players`
        vector, since the memory of the item it is pointing at will be out of scope. */
        .map(|player| player.to_string())
        .collect();
    Ok(players)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio; // For async testing
    mod parsing_tests {
        use super::{decode_multi, fetch_team_riot_id};
        const MULTI: &str = "https://www.op.gg/multisearch/na?summoners=Susurration%23ELY%2CEly+Opkillswitch%23na1%2CTuff+Daddy%23na1%2CDarklord+Ozwald%23707%2CHooked+on+Flays%232949";

        #[test]
        fn decode_player_list_test() {
            // It escapes just the commas, which might end up being fine

            let player_list = decode_multi(MULTI).expect("Failed to decode player list...");

            // Convert player_list to Vec<&str> for comparison (so that we don't have to )
            let successful_player_list: Vec<String> = vec![
                "Susurration#ELY".to_string(),
                "Ely+Opkillswitch#na1".to_string(),
                "Tuff+Daddy#na1".to_string(),
                "Darklord+Ozwald#707".to_string(),
                "Hooked+on+Flays#2949".to_string(),
            ];
            assert_eq!(player_list, successful_player_list);
        }

        /* FIXME: This test does not seem to pass. Check to see if there
        is a better "assertion" to fit the needs of this function.

        TODO: For future API tests, see if we can utilize mocking rather than physically calling the API.
        */

        #[tokio::test]
        async fn team_id_fetch_test() {
            let player_list = decode_multi(MULTI).expect("Failed to decode player list...");
            let (fetched_player_list, failed_fetches) = fetch_team_riot_id(&player_list).await;
            assert!(
                failed_fetches.is_empty(),
                "Expected failed fetches, but got {:#?}",
                failed_fetches
            );
        }
    }

    /*  FIXME: API Request Testing. Maybe add mocking?
    mod api_tests {
        use crate::{api::riot_api::fetch_riot_id, models::riot_id::parse_player_name};

        use super::*;

        fn fetch_player_test() {
            let sample_name = "Hooked on Flays#2949";
            // Would avoid using unwrap in general, but this name is syntactically valid.
            let (player_game_name, player_tag_line) = parse_player_name(sample_name).unwrap();

            let new_riot_id = fetch_riot_id(player_game_name, player_tag_line)
            assert!(fetch_riot_id(player_game_name, player_tag_line);
        }
    } */
}
