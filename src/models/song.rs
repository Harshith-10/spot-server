use crate::models::{images::Images, stream_urls::StreamUrls};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Song {
    pub seokey: String,
    pub album_seokey: Option<String>,
    pub track_id: String,
    pub title: String,
    pub artists: String,
    pub artist_seokeys: String,
    pub artist_ids: String,
    pub artist_image: Option<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub duration: Option<String>,
    pub popularity: Option<String>,
    pub genres: Option<String>,
    pub is_explicit: Option<i32>,
    pub language: Option<String>,
    pub label: Option<String>,
    pub release_date: Option<String>,
    pub play_count: Option<String>,
    pub favorite_count: Option<i32>,
    pub song_url: String,
    pub album_url: Option<String>,
    pub images: Option<Images>,
    pub stream_urls: Option<StreamUrls>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaSongResponse {
    pub tracks: Option<Vec<GaanaTrack>>,
    pub track: Option<GaanaTrack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaTrack {
    pub seokey: Option<String>,
    #[serde(rename = "albumseokey")]
    pub album_seokey: Option<String>,
    pub track_id: Option<serde_json::Value>,
    #[serde(rename = "track_title")]
    pub title: Option<String>,
    pub artist: Option<serde_json::Value>,
    #[serde(rename = "artist_detail")]
    pub artist_detail: Option<Vec<serde_json::Value>>,
    pub album_title: Option<String>,
    pub album_id: Option<serde_json::Value>,
    pub duration: Option<String>,
    pub popularity: Option<String>,
    #[serde(rename = "gener")]
    pub genre: Option<serde_json::Value>,
    #[serde(rename = "parental_warning")]
    pub explicit_content: Option<serde_json::Value>,
    pub language: Option<String>,
    #[serde(rename = "vendor_name")]
    pub label: Option<String>,
    pub release_date: Option<String>,
    pub play_ct: Option<String>,
    #[serde(rename = "total_favourite_count")]
    pub favorite_count: Option<serde_json::Value>,
    pub artwork: Option<String>,
    pub artwork_large: Option<String>,
    pub artwork_web: Option<String>,
    pub gen_url: Option<String>,
    pub album_url: Option<String>,
    pub urls: Option<GaanaStreamUrls>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaStreamUrls {
    pub medium: Option<GaanaStreamUrlQuality>,
    pub high: Option<GaanaStreamUrlQuality>,
    pub low: Option<GaanaStreamUrlQuality>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaStreamUrlQuality {
    pub message: Option<String>,
}
