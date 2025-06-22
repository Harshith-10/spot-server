use axum::{http::Method, response::Json, routing::get, Router};
use serde_json::{json, Value};
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod models;
mod utils;

use api::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::songs::search_songs,
        api::songs::get_song_info,
        api::albums::search_albums,
        api::albums::get_album_info,
        api::artists::search_artists,
        api::artists::get_artist_info,
        api::playlists::get_playlist_info,
        api::trending::get_trending,
        api::newreleases::get_new_releases,
        api::charts::get_charts,
    ),
    components(
        schemas(models::song::Song, models::album::Album, models::artist::Artist, 
                models::playlist::Playlist, models::error::ApiError, models::images::Images,
                models::stream_urls::StreamUrls)
    ),
    tags(
        (name = "songs", description = "Song search and information endpoints"),
        (name = "albums", description = "Album search and information endpoints"),
        (name = "artists", description = "Artist search and information endpoints"),
        (name = "playlists", description = "Playlist information endpoints"),
        (name = "trending", description = "Trending content endpoints"),
        (name = "newreleases", description = "New releases endpoints"),
        (name = "charts", description = "Charts endpoints")
    )
)]
struct ApiDoc;

async fn root() -> Json<Value> {
    Json(json!({
        "name": "Spot API",
        "version": "1.0.0",
        "description": "Unofficial JSON API for Gaana.com - Indian Music Streaming Service",
        "documentation": "/docs",
        "endpoints": {
            "songs": {
                "search": "/songs/search",
                "info": "/songs/info"
            },
            "albums": {
                "search": "/albums/search",
                "info": "/albums/info"
            },
            "artists": {
                "search": "/artists/search",
                "info": "/artists/info"
            },
            "playlists": {
                "info": "/playlists/info"
            },
            "trending": "/trending",
            "newreleases": "/newreleases",
            "charts": "/charts"
        }
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spot_server_v2=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get port from environment or default to 8000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        // Song endpoints (with and without trailing slash for compatibility)
        .route("/songs/search", get(songs::search_songs))
        .route("/songs/search/", get(songs::search_songs))
        .route("/songs/info", get(songs::get_song_info))
        .route("/songs/info/", get(songs::get_song_info))
        // Album endpoints
        .route("/albums/search", get(albums::search_albums))
        .route("/albums/search/", get(albums::search_albums))
        .route("/albums/info", get(albums::get_album_info))
        .route("/albums/info/", get(albums::get_album_info))
        // Artist endpoints
        .route("/artists/search", get(artists::search_artists))
        .route("/artists/search/", get(artists::search_artists))
        .route("/artists/info", get(artists::get_artist_info))
        .route("/artists/info/", get(artists::get_artist_info))
        // Playlist endpoints
        .route("/playlists/info", get(playlists::get_playlist_info))
        .route("/playlists/info/", get(playlists::get_playlist_info))
        // Trending, New Releases, Charts
        .route("/trending", get(trending::get_trending))
        .route("/trending/", get(trending::get_trending))
        .route("/newreleases", get(newreleases::get_new_releases))
        .route("/newreleases/", get(newreleases::get_new_releases))
        .route("/charts", get(charts::get_charts))
        .route("/charts/", get(charts::get_charts))
        // Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                        .allow_headers(Any)
                        .expose_headers(Any),
                ),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    tracing::info!("🎶 Spot API server running on http://localhost:{}", port);
    tracing::info!(
        "📚 API Documentation available at http://localhost:{}/docs",
        port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
