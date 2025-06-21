use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{artist::*, error::ApiError};

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchArtistsQuery {
    query: String,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ArtistInfoQuery {
    seokey: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ArtistResponse {
    Artists(Vec<Artist>),
    Artist(Artist),
    Error(ApiError),
}

/// Search for artists by name
#[utoipa::path(
    get,
    path = "/artists/search",
    params(SearchArtistsQuery),
    responses(
        (status = 200, description = "List of artists matching the search query", body = Vec<Artist>),
        (status = 404, description = "No artists found", body = ApiError)
    ),
    tag = "artists"
)]
pub async fn search_artists(
    Query(params): Query<SearchArtistsQuery>,
) -> Result<Json<ArtistResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    // Use the exact same endpoint as the Python version
    let search_url = format!(
        "https://gaana.com/apiv2?country=IN&page=0&secType=artist&type=search&keyword={}",
        urlencoding::encode(&params.query)
    );

    eprintln!("Trying artist search endpoint: {}", search_url);
    match api.make_request_flexible(&search_url).await {
        Ok(response) => {
            eprintln!("Got artist search response, extracting artist IDs...");

            // Extract artist seokeys from search response like the Python version does
            let mut artist_ids = Vec::new();
            let limit = params.limit.unwrap_or(10);

            // Parse the search response to get seokeys - same structure as songs and albums
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
                                            artist_ids.push(seo_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if artist_ids.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found("No artists found for the given query")),
                ));
            }

            eprintln!("Found {} artist IDs: {:?}", artist_ids.len(), artist_ids);

            // Now get artist info for each seokey, just like Python version
            let mut artists = Vec::new();
            for artist_id in artist_ids {
                let detail_url = format!(
                    "https://gaana.com/apiv2?type=artistDetail&seokey={}",
                    urlencoding::encode(&artist_id)
                );

                eprintln!("Getting details for artist: {}", artist_id);
                match api.make_request_flexible(&detail_url).await {
                    Ok(detail_response) => {
                        if let Some(processed_artist) =
                            api.process_gaana_artist_response(&detail_response, false)
                        {
                            artists.push(processed_artist);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get details for artist {}: {}", artist_id, e);
                        // Continue with other artists
                    }
                }
            }

            if artists.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found("No valid artist data found")),
                ));
            }

            Ok(Json(ArtistResponse::Artists(artists)))
        }
        Err(e) => {
            eprintln!("Artist search request error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ));
        }
    }
}

/// Get detailed information about a specific artist
#[utoipa::path(
    get,
    path = "/artists/info",
    params(ArtistInfoQuery),
    responses(
        (status = 200, description = "Detailed information about the artist", body = Artist),
        (status = 404, description = "Artist not found", body = ApiError)
    ),
    tag = "artists"
)]
pub async fn get_artist_info(
    Query(params): Query<ArtistInfoQuery>,
) -> Result<Json<ArtistResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    let url = format!(
        "https://gaana.com/apiv2?type=artistDetail&seokey={}",
        urlencoding::encode(&params.seokey)
    );

    eprintln!("Getting artist info from: {}", url);
    match api.make_request_flexible(&url).await {
        Ok(response) => {
            eprintln!("Parsing artist info response...");

            if let Some(processed_artist) = api.process_gaana_artist_response(&response, true) {
                return Ok(Json(ArtistResponse::Artist(processed_artist)));
            }

            eprintln!("No valid artist found in response");
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
            eprintln!("Artist info request failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ))
        }
    }
}
