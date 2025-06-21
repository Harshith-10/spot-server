use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::api::base::BaseApi;
use crate::models::{album::*, error::ApiError};

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchAlbumsQuery {
    query: String,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct AlbumInfoQuery {
    seokey: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AlbumResponse {
    Albums(Vec<Album>),
    Album(Album),
    Error(ApiError),
}

/// Search for albums by name
#[utoipa::path(
    get,
    path = "/albums/search",
    params(SearchAlbumsQuery),
    responses(
        (status = 200, description = "List of albums matching the search query", body = Vec<Album>),
        (status = 404, description = "No albums found", body = ApiError)
    ),
    tag = "albums"
)]
pub async fn search_albums(
    Query(params): Query<SearchAlbumsQuery>,
) -> Result<Json<AlbumResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    // Use the exact same endpoint as the Python version
    let search_url = format!(
        "https://gaana.com/apiv2?country=IN&page=0&secType=album&type=search&keyword={}",
        urlencoding::encode(&params.query)
    );

    eprintln!("Trying album search endpoint: {}", search_url);
    match api.make_request_flexible(&search_url).await {
        Ok(response) => {
            eprintln!("Got album search response, extracting album IDs...");

            // Extract album seokeys from search response like the Python version does
            let mut album_ids = Vec::new();
            let limit = params.limit.unwrap_or(10);

            // Parse the search response to get seokeys - same structure as songs
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
                                            album_ids.push(seo_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if album_ids.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found("No albums found for the given query")),
                ));
            }

            eprintln!("Found {} album IDs: {:?}", album_ids.len(), album_ids);

            // Now get album info for each seokey, just like Python version
            let mut albums = Vec::new();
            for album_id in album_ids {
                let detail_url = format!(
                    "https://gaana.com/apiv2?type=albumDetail&seokey={}",
                    urlencoding::encode(&album_id)
                );

                eprintln!("Getting details for album: {}", album_id);
                match api.make_request_flexible(&detail_url).await {
                    Ok(detail_response) => {
                        if let Some(processed_album) =
                            api.process_gaana_album_response(&detail_response, false)
                        {
                            albums.push(processed_album);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get details for album {}: {}", album_id, e);
                        // Continue with other albums
                    }
                }
            }

            if albums.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiError::not_found("No valid album data found")),
                ));
            }

            Ok(Json(AlbumResponse::Albums(albums)))
        }
        Err(e) => {
            eprintln!("Album search request error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ));
        }
    }
}

/// Get detailed information about a specific album
#[utoipa::path(
    get,
    path = "/albums/info",
    params(AlbumInfoQuery),
    responses(
        (status = 200, description = "Detailed information about the album", body = Album),
        (status = 404, description = "Album not found", body = ApiError)
    ),
    tag = "albums"
)]
pub async fn get_album_info(
    Query(params): Query<AlbumInfoQuery>,
) -> Result<Json<AlbumResponse>, (StatusCode, Json<ApiError>)> {
    let api = BaseApi::new();

    let url = format!(
        "https://gaana.com/apiv2?type=albumDetail&seokey={}",
        urlencoding::encode(&params.seokey)
    );

    eprintln!("Getting album info from: {}", url);
    match api.make_request_flexible(&url).await {
        Ok(response) => {
            eprintln!("Parsing album info response...");

            if let Some(processed_album) = api.process_gaana_album_response(&response, true) {
                return Ok(Json(AlbumResponse::Album(processed_album)));
            }

            eprintln!("No valid album found in response");
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
            eprintln!("Album info request failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::internal_error(&format!("Request failed: {}", e))),
            ))
        }
    }
}
