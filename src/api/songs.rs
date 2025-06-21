use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{error::ApiError, song::*};

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchSongsQuery {
    query: String,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct SongInfoQuery {
    seokey: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SongResponse {
    Songs(Vec<Song>),
    Song(Song),
    Error(ApiError),
}

/// Search for songs by name/title
#[utoipa::path(
    get,
    path = "/songs/search",
    params(SearchSongsQuery),
    responses(
        (status = 200, description = "List of songs matching the search query", body = Vec<Song>),
        (status = 404, description = "No songs found", body = ApiError)
    ),
    tag = "songs"
)]
pub async fn search_songs(
    Query(params): Query<SearchSongsQuery>,
) -> Result<Json<SongResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    // Use the exact same endpoint as the Python version
    let search_url = format!(
        "https://gaana.com/apiv2?country=IN&page=0&secType=track&type=search&keyword={}",
        urlencoding::encode(&params.query)
    );

    eprintln!("Trying search endpoint: {}", search_url);
    match api.make_request_flexible(&search_url).await {
        Ok(response) => {
            eprintln!("Got search response, extracting track IDs...");

            // Extract track IDs from search response like the Python version does
            let mut track_ids = Vec::new();
            let limit = params.limit.unwrap_or(10);

            // Parse the search response to get seokeys
            if let Some(gr) = response.get("gr") {
                if let Some(gr_array) = gr.as_array() {
                    if let Some(first_group) = gr_array.first() {
                        if let Some(gd) = first_group.get("gd") {
                            if let Some(gd_array) = gd.as_array() {
                                for (i, item) in gd_array.iter().enumerate() {
                                    if i >= limit {
                                        break;
                                    }
                                    if let Some(seo) = item.get("seo") {
                                        if let Some(seo_str) = seo.as_str() {
                                            track_ids.push(seo_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if track_ids.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found("No songs found for the given query")),
                ));
            }

            eprintln!("Found {} track IDs: {:?}", track_ids.len(), track_ids);

            // Now get track info for each seokey, just like Python version
            let mut songs = Vec::new();
            for track_id in track_ids {
                let detail_url = format!(
                    "https://gaana.com/apiv2?type=songDetail&seokey={}",
                    urlencoding::encode(&track_id)
                );

                eprintln!("Getting details for track: {}", track_id);
                match api.make_request_flexible(&detail_url).await {
                    Ok(detail_response) => {
                        if let Some(tracks) = detail_response.get("tracks") {
                            if let Some(tracks_array) = tracks.as_array() {
                                for track_data in tracks_array {
                                    let gaana_track: Result<GaanaTrack, _> =
                                        serde_json::from_value(track_data.clone());
                                    if let Ok(track) = gaana_track {
                                        if let Some(song) = api.process_gaana_track(&track) {
                                            songs.push(song);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get details for track {}: {}", track_id, e);
                        // Continue with other tracks
                    }
                }
            }

            if songs.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found("No valid song data found")),
                ));
            }

            Ok(Json(SongResponse::Songs(songs)))
        }
        Err(e) => {
            eprintln!("Search request error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ));
        }
    }
}

/// Get detailed information about a specific song
#[utoipa::path(
    get,
    path = "/songs/info",
    params(SongInfoQuery),
    responses(
        (status = 200, description = "Detailed information about the song", body = Song),
        (status = 404, description = "Song not found", body = ApiError)
    ),
    tag = "songs"
)]
pub async fn get_song_info(
    Query(params): Query<SongInfoQuery>,
) -> Result<Json<SongResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();
    let url = format!(
        "https://gaana.com/apiv2?type=songDetail&seokey={}",
        urlencoding::encode(&params.seokey)
    );

    eprintln!("Getting song info from: {}", url);
    match api.make_request_flexible(&url).await {
        Ok(response) => {
            eprintln!("Parsing song info response...");

            // Look for 'tracks' array just like in the search function
            if let Some(tracks) = response.get("tracks") {
                if let Some(tracks_array) = tracks.as_array() {
                    if let Some(track_data) = tracks_array.first() {
                        let gaana_track: Result<GaanaTrack, _> =
                            serde_json::from_value(track_data.clone());
                        if let Ok(track) = gaana_track {
                            if let Some(song) = api.process_gaana_track(&track) {
                                return Ok(Json(SongResponse::Song(song)));
                            }
                        }
                    }
                }
            }

            eprintln!("No valid track found in response");
            eprintln!(
                "Response structure: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            );

            Err((
                StatusCode::NOT_FOUND,
                Json(ApiError::invalid_seokey(&params.seokey)),
            ))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ))
        }
    }
}
