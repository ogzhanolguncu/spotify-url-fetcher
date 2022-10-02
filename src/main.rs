use dotenv::dotenv;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use search_spotify::{print_tracks, APIResponse};
use std::env;
use tokio;

const END_POINT: &str = "https://api.spotify.com/v1/search";

fn main() {
    dotenv().ok();

    let spotify_secret_key = env::var("SPOTIFY_SECRET_KEY").unwrap().to_string();
    let url = format!(
        "{END_POINT}?q={query}&type=track,artist",
        query = "Aka Gunduz Kutbay",
    );

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let client = reqwest::Client::new();

    rt.block_on(async {
        let response = client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {spotify_secret_key}"))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            // each response is wrapped in a `Result` type
            // we'll unwrap here for simplicity
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => eprintln!("Something went wrong"),
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            unknown => {
                panic!("Dunno, whats wromg with the code {:?}", unknown);
            }
        }
    });
}
