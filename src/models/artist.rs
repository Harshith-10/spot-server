use crate::models::images::Images;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Artist {
    pub seokey: String,
    pub artist_id: String,
    pub name: String,
    pub language: Option<String>,
    pub play_count: Option<String>,
    pub favorite_count: Option<i32>,
    pub artist_url: String,
    pub images: Option<Images>,
    pub top_tracks: Option<Vec<crate::models::song::Song>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaArtistResponse {
    pub artists: Option<Vec<GaanaArtist>>,
    pub artist: Option<GaanaArtist>,
    pub tracks: Option<Vec<crate::models::song::GaanaTrack>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaArtist {
    pub seokey: Option<String>,
    pub artist_id: Option<serde_json::Value>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub play_ct: Option<String>,
    pub favorite_count: Option<serde_json::Value>,
    pub artwork: Option<String>,
    pub artwork_large: Option<String>,
    pub artwork_web: Option<String>,
    pub gen_url: Option<String>,
}
