use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{error::ApiError, song::*};

#[derive(Debug, Deserialize, IntoParams)]
pub struct TrendingQuery {
    #[serde(alias = "lang")]
    language: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum TrendingResponse {
    Songs(Vec<Song>),
    Error(ApiError),
}

/// Get trending songs by language
#[utoipa::path(
    get,
    path = "/trending",
    params(TrendingQuery),
    responses(
        (status = 200, description = "List of trending songs", body = Vec<Song>),
        (status = 404, description = "No trending songs found", body = ApiError)
    ),
    tag = "trending"
)]
pub async fn get_trending(
    Query(params): Query<TrendingQuery>,
) -> Result<Json<TrendingResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    eprintln!("Trending request params: {:?}", params);

    // Use the language as-is, just like the Python version
    let language = params.language.unwrap_or_else(|| "English".to_string());
    let limit = params.limit.unwrap_or(20);

    // Use the exact same approach as the working Python version
    let url = "https://gaana.com/apiv2?type=miscTrendingSongs";
    eprintln!("Trying a request to: {}", url);

    let client = reqwest::Client::new();

    // POST request with simple cookie like Python version
    let response = client
        .post(url)
        .header("Cookie", format!("__ul={}", language))
        .header("Content-Length", "0")
        .body("")
        .send()
        .await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::internal_error(&format!(
                        "HTTP request failed with status: {}",
                        resp.status()
                    ))),
                ));
            }

            let response_text = resp.text().await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::internal_error(&format!(
                        "Failed to read response: {}",
                        e
                    ))),
                )
            })?;

            let json_response: serde_json::Value =
                serde_json::from_str(&response_text).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiError::internal_error(&format!(
                            "Failed to parse JSON: {}",
                            e
                        ))),
                    )
                })?;

            process_trending_response(json_response, limit, api).await
        }
        Err(e) => {
            eprintln!("Trending request error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ))
        }
    }
}

async fn process_trending_response(
    json_response: serde_json::Value,
    limit: usize,
    api: BaseApi,
) -> Result<Json<TrendingResponse>, (StatusCode, Json<ApiError>)> {
    eprintln!("Got trending response, extracting track seokeys...");

    // Extract track seokeys from entities array like the Python version
    let mut track_seokeys = Vec::new();

    if let Some(entities) = json_response.get("entities") {
        if let Some(entities_array) = entities.as_array() {
            for (i, entity) in entities_array.iter().enumerate() {
                if i >= limit {
                    break;
                }
                if let Some(seokey) = entity.get("seokey") {
                    if let Some(seokey_str) = seokey.as_str() {
                        track_seokeys.push(seokey_str.to_string());
                    }
                }
            }
        }
    }

    if track_seokeys.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiError::not_found("No trending songs found")),
        ));
    }

    eprintln!(
        "Found {} trending track seokeys: {:?}",
        track_seokeys.len(),
        track_seokeys
    );

    // Now get track info for each seokey
    let mut songs = Vec::new();
    for track_id in track_seokeys {
        let detail_url = format!(
            "https://gaana.com/apiv2?type=songDetail&seokey={}",
            urlencoding::encode(&track_id)
        );

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
            Json(ApiError::not_found("No valid trending song data found")),
        ));
    }

    Ok(Json(TrendingResponse::Songs(songs)))
}
