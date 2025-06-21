use axum::{extract::Query, http::StatusCode, response::Json};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{error::ApiError, song::Song};
use urlencoding::encode;
use serde_json;

#[derive(Debug, Deserialize, IntoParams)]
pub struct PlaylistInfoQuery {
    seokey: String,
}

/// Get detailed information about a specific playlist
#[utoipa::path(
    get,
    path = "/playlists/info",
    params(PlaylistInfoQuery),
    responses(
        (status = 200, description = "List of songs in the playlist", body = Vec<Song>),
        (status = 404, description = "Playlist not found", body = ApiError)
    ),
    tag = "playlists"
)]
pub async fn get_playlist_info(
    Query(params): Query<PlaylistInfoQuery>,
) -> Result<Json<Vec<Song>>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();
    // Build playlist detail URL
    let url = format!(
        "https://gaana.com/apiv2?type=playlistDetail&seokey={}",
        encode(&params.seokey)
    );

    // Fetch playlist details
    let response = match api.make_request_flexible(&url).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to get playlist info: {}", e);
            return Err((
                StatusCode::NOT_FOUND,
                Json(ApiError::not_found(&format!(
                    "Playlist with seokey '{}' not found",
                    params.seokey
                ))),
            ));
        }
    };

    // Extract seokeys from playlist tracks
    let mut track_seokeys = Vec::new();
    if let Some(tracks) = response.get("tracks").and_then(|t| t.as_array()) {
        for track in tracks {
            if let Some(seo) = track.get("seokey").and_then(|s| s.as_str()) {
                track_seokeys.push(seo.to_string());
            }
        }
    }
    if track_seokeys.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiError::not_found(&format!(
                "No tracks found for playlist '{}'",
                params.seokey
            ))),
        ));
    }
    
    // Fetch detailed info for each track
    let mut songs = Vec::new();
    for seokey in track_seokeys {
        let detail_url = format!(
            "https://gaana.com/apiv2?type=songDetail&seokey={}",
            encode(&seokey)
        );
        match api.make_request_flexible(&detail_url).await {
            Ok(detail_response) => {
                if let Some(arr) = detail_response.get("tracks").and_then(|t| t.as_array()) {
                    if let Some(item) = arr.first() {
                        if let Ok(track) = serde_json::from_value::<crate::models::song::GaanaTrack>(
                            item.clone()
                        ) {
                            if let Some(song) = api.process_gaana_track(&track) {
                                songs.push(song);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to get details for track {}: {}", seokey, e);
            }
        }
    }
    if songs.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiError::not_found(&format!(
                "No valid song data found for playlist '{}'",
                params.seokey
            ))),
        ));
    }
    Ok(Json(songs))
}
