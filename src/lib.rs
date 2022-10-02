use serde::{Deserialize, Serialize};

mod api_response {}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Items<T> {
    pub items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub tracks: Items<Track>,
}

pub fn print_tracks(tracks: Vec<&Track>) {
    tracks.iter().for_each(|track| {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    })
}
