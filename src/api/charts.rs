use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{error::ApiError, playlist::*, images::Images};
use crate::utils::formatting;

#[derive(Debug, Deserialize, IntoParams)]
pub struct ChartsQuery {
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChartsResponse {
    Playlists(Vec<Playlist>),
    Error(ApiError),
}

/// Get current top charts (list of popular playlists)
#[utoipa::path(
    get,
    path = "/charts",
    params(ChartsQuery),
    responses(
        (status = 200, description = "List of top charts playlists", body = Vec<Playlist>),
        (status = 404, description = "No charts found", body = ApiError)
    ),
    tag = "charts"
)]
pub async fn get_charts(
    Query(params): Query<ChartsQuery>,
) -> Result<Json<ChartsResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    let url = "https://gaana.com/apiv2?page=0&type=miscTopCharts";

    match api.make_request(url).await {
        Ok(response) => {
            let charts_response: Result<GaanaChartsResponse, _> =
                serde_json::from_value(response.clone());

            match charts_response {
                Ok(data) => {
                    if let Some(entities) = data.entities {
                        let mut playlist_list = Vec::new();
                        
                        let limit = params.limit.unwrap_or(10);
                        let entities_to_process = entities.into_iter().take(limit);

                        for entity in entities_to_process {
                            if let Some(processed_playlist) = format_chart_entity(&entity) {
                                playlist_list.push(processed_playlist);
                            }
                        }

                        if playlist_list.is_empty() {
                            return Err((
                                StatusCode::NOT_FOUND,
                                Json(ApiError::not_found("top charts")),
                            ));
                        }

                        Ok(Json(ChartsResponse::Playlists(playlist_list)))
                    } else {
                        Err((
                            StatusCode::NOT_FOUND,
                            Json(ApiError::not_found("top charts entities")),
                        ))
                    }
                }
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::internal_error("Failed to parse charts response")),
                )),
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&format!("Request failed: {}", e))),
        )),
    }
}

// Helper function to format chart entity similar to Python's format_json_charts
fn format_chart_entity(entity: &GaanaChartEntity) -> Option<Playlist> {
    let seokey = entity.seokey.as_ref()?.clone();
    let playlist_id = formatting::extract_id(&entity.entity_id);
    let title = entity.name.as_ref()?.clone();
    
    // Extract is_explicit from entity_info[6] if available (unused in current implementation but available for future use)
    let _is_explicit = entity.entity_info
        .as_ref()
        .and_then(|info| info.get(6))
        .and_then(|item| item.value.as_ref())
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Extract play_count from last entity_info item if available
    let play_count = entity.entity_info
        .as_ref()
        .and_then(|info| info.last())
        .and_then(|item| item.value.as_ref())
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // Create images from atwj
    let images = if let Some(artwork_url) = &entity.atwj {
        Some(Images::new(
            Some(artwork_url.replace("size_m.jpg", "size_l.jpg")),
            Some(artwork_url.clone()),
            Some(artwork_url.replace("size_m.jpg", "size_s.jpg")),
        ))
    } else {
        None
    };

    Some(Playlist {
        seokey: seokey.clone(),
        playlist_id,
        title,
        description: None,
        language: entity.language.clone(),
        play_count,
        favorite_count: formatting::extract_int(&entity.favorite_count),
        playlist_url: format!("https://gaana.com/playlist/{}", seokey),
        images,
        total_tracks: None,
        tracks: None,
    })
}
