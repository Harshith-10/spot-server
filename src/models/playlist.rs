use crate::models::images::Images;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Playlist {
    pub seokey: String,
    pub playlist_id: String,
    pub title: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub play_count: Option<String>,
    pub favorite_count: Option<i32>,
    pub playlist_url: String,
    pub images: Option<Images>,
    pub total_tracks: Option<i32>,
    pub tracks: Option<Vec<crate::models::song::Song>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaPlaylistResponse {
    pub playlist: Option<GaanaPlaylist>,
    pub tracks: Option<Vec<crate::models::song::GaanaTrack>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaPlaylist {
    pub seokey: Option<String>,
    pub playlist_id: Option<serde_json::Value>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub play_ct: Option<String>,
    pub favorite_count: Option<serde_json::Value>,
    pub artwork: Option<String>,
    pub artwork_large: Option<String>,
    pub artwork_web: Option<String>,
    pub gen_url: Option<String>,
    pub total_tracks: Option<serde_json::Value>,
}

// Charts-specific response structure based on Python reference
#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaChartsResponse {
    pub entities: Option<Vec<GaanaChartEntity>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaChartEntity {
    pub seokey: Option<String>,
    pub entity_id: Option<serde_json::Value>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub favorite_count: Option<serde_json::Value>,
    pub entity_info: Option<Vec<GaanaEntityInfo>>,
    pub atwj: Option<String>, // artwork URL
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GaanaEntityInfo {
    pub value: Option<serde_json::Value>,
}
