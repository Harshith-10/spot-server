use crate::models::images::Images;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Album {
    pub seokey: String,
    pub album_id: String,
    pub title: String,
    pub artists: String,
    pub artist_seokeys: String,
    pub artist_ids: String,
    pub language: Option<String>,
    pub label: Option<String>,
    pub release_date: Option<String>,
    pub play_count: Option<String>,
    pub favorite_count: Option<i32>,
    pub album_url: String,
    pub images: Option<Images>,
    pub total_tracks: Option<i32>,
    pub tracks: Option<Vec<crate::models::song::Song>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaAlbumResponse {
    pub albums: Option<Vec<GaanaAlbum>>,
    pub album: Option<GaanaAlbum>,
    pub tracks: Option<Vec<crate::models::song::GaanaTrack>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaAlbum {
    pub seokey: Option<String>,
    pub album_id: Option<serde_json::Value>,
    pub title: Option<String>,
    pub artist: Option<serde_json::Value>,
    pub language: Option<String>,
    pub label: Option<String>,
    pub release_date: Option<String>,
    pub play_ct: Option<String>,
    pub favorite_count: Option<serde_json::Value>,
    pub artwork: Option<String>,
    pub artwork_large: Option<String>,
    pub artwork_web: Option<String>,
    pub gen_url: Option<String>,
    pub total_tracks: Option<serde_json::Value>,
}
