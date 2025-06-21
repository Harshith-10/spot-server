use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{error::ApiError, song::*, album::Album};
use crate::utils::formatting;

#[derive(Debug, Deserialize, IntoParams)]
pub struct NewReleasesQuery {
    #[serde(rename = "lang")]
    language: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct NewReleasesResponse {
    pub songs: Vec<Song>,
    pub albums: Vec<Album>,
}

/// Get new song and album releases by language
#[utoipa::path(
    get,
    path = "/newreleases",
    params(NewReleasesQuery),
    responses(
        (status = 200, description = "List of new releases", body = NewReleasesResponse),
        (status = 404, description = "No new releases found", body = ApiError)
    ),
    tag = "newreleases"
)]
pub async fn get_new_releases(
    Query(params): Query<NewReleasesQuery>,
) -> Result<Json<NewReleasesResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    // Validate and normalize language
    let language = match params.language {
        Some(lang) => formatting::validate_language(&lang),
        None => "Hindi".to_string(),
    };

    let url = format!(
        "https://gaana.com/apiv2?page=0&type=miscNewRelease&language={}",
        urlencoding::encode(&language)
    );

    match api.make_request_flexible(&url).await {
        Ok(response) => {
            // Gather track and album seokeys from entities
            let limit = params.limit.unwrap_or(10);
            let mut track_seokeys = Vec::new();
            let mut album_seokeys = Vec::new();
            if let Some(entities) = response.get("entities").and_then(|v| v.as_array()) {
                for (i, entity) in entities.iter().enumerate() {
                    if i >= limit {
                        break;
                    }
                    if let (Some(entity_type), Some(seokey)) = (
                        entity.get("entity_type").and_then(|v| v.as_str()),
                        entity.get("seokey").and_then(|v| v.as_str()),
                    ) {
                        match entity_type {
                            "TR" => track_seokeys.push(seokey.to_string()),
                            "AL" => album_seokeys.push(seokey.to_string()),
                            _ => {}
                        }
                    }
                }
            }
            // Return 404 if no results
            if track_seokeys.is_empty() && album_seokeys.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found(&format!("new releases in {}", language))),
                ));
            }
            // Fetch track details
            let mut songs = Vec::new();
            for key in track_seokeys {
                let detail_url = format!(
                    "https://gaana.com/apiv2?type=songDetail&seokey={}",
                    urlencoding::encode(&key)
                );
                if let Ok(detail_resp) = api.make_request_flexible(&detail_url).await {
                    if let Some(tracks) = detail_resp.get("tracks").and_then(|v| v.as_array()) {
                        for track_data in tracks {
                            if let Ok(track) = serde_json::from_value::<GaanaTrack>(track_data.clone()) {
                                if let Some(song) = api.process_gaana_track(&track) {
                                    songs.push(song);
                                }
                            }
                        }
                    }
                }
            }
            // Fetch album details
            let mut albums = Vec::new();
            for key in album_seokeys {
                let detail_url = format!(
                    "https://gaana.com/apiv2?type=albumDetail&seokey={}",
                    urlencoding::encode(&key)
                );
                if let Ok(detail_resp) = api.make_request_flexible(&detail_url).await {
                    if let Some(processed) = api.process_gaana_album_response(&detail_resp, false) {
                        albums.push(processed);
                    }
                }
            }
            Ok(Json(NewReleasesResponse { songs, albums }))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&format!("Request failed: {}", e))),
        )),
    }
}
