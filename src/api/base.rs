use crate::models::{album::*, artist::*, images::Images, playlist::*, song::*};
use crate::utils::{encryption, formatting};
use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

pub struct BaseApi {
    client: Client,
}

impl BaseApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    pub async fn make_request(&self, url: &str) -> Result<Value> {
        let response = self
            .client
            .post(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Content-Length", "0")
            .send()
            .await?;

        // Check response status
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }

        // Get response text first for better error diagnostics
        let response_text = response.text().await?;

        // Log the first few characters for debugging
        if response_text.len() > 200 {
            eprintln!("Response preview: {}", &response_text[..200]);
        } else {
            eprintln!("Full response: {}", response_text);
        }

        // Try to parse as JSON
        let json_value: Value = serde_json::from_str(&response_text)
            .map_err(|e| anyhow::anyhow!("error decoding response body: {}", e))?;

        Ok(json_value)
    }
    /// More flexible method to handle different possible response structures
    pub async fn make_request_flexible(&self, url: &str) -> Result<Value> {
        let response = self
            .client
            .post(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept", "application/json, text/plain, */*")
            .header("Content-Length", "0")
            .header("Referer", "https://gaana.com/")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }

        let response_text = response.text().await?;

        if response_text.trim().is_empty() {
            return Err(anyhow::anyhow!("Empty response from server"));
        }

        let trimmed = response_text.trim();
        if !trimmed.starts_with('{') && !trimmed.starts_with('[') {
            if trimmed.contains("<html") || trimmed.contains("<!DOCTYPE") {
                return Err(anyhow::anyhow!(
                    "Server returned HTML instead of JSON. Possible API endpoint change."
                ));
            }
            return Err(anyhow::anyhow!(
                "Response is not valid JSON format: {}",
                &trimmed[..std::cmp::min(100, trimmed.len())]
            ));
        }

        match serde_json::from_str::<Value>(&response_text) {
            Ok(json_value) => Ok(json_value),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to parse JSON response: {}. Response text: {}",
                e,
                response_text
            )),
        }
    }

    pub fn process_song_response(&self, response: &Value) -> Vec<Song> {
        let mut songs = Vec::new();
        if let Some(tracks) = response.get("tracks").and_then(|t| t.as_array()) {
            for track_data in tracks {
                let gaana_track: Result<GaanaTrack, _> = serde_json::from_value(track_data.clone());
                if let Ok(track) = gaana_track {
                    if let Some(song) = self.process_gaana_track(&track) {
                        songs.push(song);
                    }
                }
            }
        }
        songs
    }

    pub fn process_gaana_track(&self, track: &GaanaTrack) -> Option<Song> {
        let seokey = track.seokey.as_ref()?.clone();
        let track_id = formatting::extract_id(&track.track_id);
        let title = track.title.as_ref()?.clone();

        // Process artists
        let (artists, artist_seokeys, artist_ids, artist_image) =
            formatting::process_artists(&track.artist);

        // Get artist image from artist_detail if available
        let artist_image = if let Some(artist_detail) = &track.artist_detail {
            if let Some(first_artist) = artist_detail.first() {
                if let Some(atw) = first_artist.get("atw") {
                    atw.as_str().map(|s| s.to_string())
                } else {
                    artist_image
                }
            } else {
                artist_image
            }
        } else {
            artist_image
        };

        // Create images
        let images = if let Some(artwork) = &track.artwork {
            Some(Images::new(
                track.artwork_large.clone(),
                Some(artwork.clone()),
                track.artwork_web.clone(),
            ))
        } else {
            None
        };

        // Process stream URLs if available
        let stream_urls = if let Some(urls) = &track.urls {
            if let Some(medium) = &urls.medium {
                if let Some(encrypted_url) = &medium.message {
                    Some(encryption::decrypt_stream_url(encrypted_url))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Some(Song {
            seokey: seokey.clone(),
            album_seokey: track.album_seokey.clone(),
            track_id,
            title,
            artists,
            artist_seokeys,
            artist_ids,
            artist_image,
            album: track.album_title.clone(),
            album_id: Some(formatting::extract_id(&track.album_id)),
            duration: track.duration.clone(),
            popularity: track.popularity.clone(),
            genres: formatting::process_genres(&track.genre),
            is_explicit: formatting::extract_int(&track.explicit_content),
            language: track.language.clone(),
            label: track.label.clone(),
            release_date: track.release_date.clone(),
            play_count: track.play_ct.clone(),
            favorite_count: formatting::extract_int(&track.favorite_count),
            song_url: track
                .gen_url
                .clone()
                .unwrap_or_else(|| format!("https://gaana.com/song/{}", seokey)),
            album_url: track.album_url.clone(),
            images,
            stream_urls,
        })
    }

    pub fn process_gaana_album(
        &self,
        album: &GaanaAlbum,
        tracks: Option<Vec<Song>>,
    ) -> Option<Album> {
        let seokey = album.seokey.as_ref()?.clone();
        let album_id = formatting::extract_id(&album.album_id);
        let title = album.title.as_ref()?.clone();

        // Process artists
        let (artists, artist_seokeys, artist_ids) =
            formatting::process_album_artists(&album.artist);

        // Create images
        let images = if let Some(artwork) = &album.artwork {
            Some(Images::new(
                album.artwork_large.clone(),
                Some(artwork.clone()),
                album.artwork_web.clone(),
            ))
        } else {
            None
        };
        Some(Album {
            seokey: seokey.clone(),
            album_id,
            title,
            artists,
            artist_seokeys,
            artist_ids,
            language: album.language.clone(),
            label: album.label.clone(),
            release_date: album.release_date.clone(),
            play_count: album.play_ct.clone(),
            favorite_count: formatting::extract_int(&album.favorite_count),
            album_url: album
                .gen_url
                .clone()
                .unwrap_or_else(|| format!("https://gaana.com/album/{}", seokey)),
            images,
            total_tracks: formatting::extract_int(&album.total_tracks),
            tracks,
        })
    }

    pub fn process_gaana_artist(
        &self,
        artist: &GaanaArtist,
        top_tracks: Option<Vec<Song>>,
    ) -> Option<Artist> {
        let seokey = artist.seokey.as_ref()?.clone();
        let artist_id = formatting::extract_id(&artist.artist_id);
        let name = artist.name.as_ref()?.clone();

        // Create images
        let images = if let Some(artwork) = &artist.artwork {
            Some(Images::new(
                artist.artwork_large.clone(),
                Some(artwork.clone()),
                artist.artwork_web.clone(),
            ))
        } else {
            None
        };
        Some(Artist {
            seokey: seokey.clone(),
            artist_id,
            name,
            language: artist.language.clone(),
            play_count: artist.play_ct.clone(),
            favorite_count: formatting::extract_int(&artist.favorite_count),
            artist_url: artist
                .gen_url
                .clone()
                .unwrap_or_else(|| format!("https://gaana.com/artist/{}", seokey)),
            images,
            top_tracks,
        })
    }

    pub fn process_gaana_playlist(
        &self,
        playlist: &GaanaPlaylist,
        tracks: Option<Vec<Song>>,
    ) -> Option<Playlist> {
        let seokey = playlist.seokey.as_ref()?.clone();
        let playlist_id = formatting::extract_id(&playlist.playlist_id);
        let title = playlist.title.as_ref()?.clone();

        // Create images
        let images = if let Some(artwork) = &playlist.artwork {
            Some(Images::new(
                playlist.artwork_large.clone(),
                Some(artwork.clone()),
                playlist.artwork_web.clone(),
            ))
        } else {
            None
        };
        Some(Playlist {
            seokey: seokey.clone(),
            playlist_id,
            title,
            description: playlist.description.clone(),
            language: playlist.language.clone(),
            play_count: playlist.play_ct.clone(),
            favorite_count: formatting::extract_int(&playlist.favorite_count),
            playlist_url: playlist
                .gen_url
                .clone()
                .unwrap_or_else(|| format!("https://gaana.com/playlist/{}", seokey)),
            images,
            total_tracks: formatting::extract_int(&playlist.total_tracks),
            tracks,
        })
    }
    /// Process album response from the API just like the Python version
    pub fn process_gaana_album_response(
        &self,
        response: &Value,
        include_tracks: bool,
    ) -> Option<Album> {
        // The Python code expects: result['album'] and result['tracks']
        let album_data = response.get("album")?;

        let seokey = album_data.get("seokey")?.as_str()?.to_string();
        let album_id = formatting::extract_id(&album_data.get("album_id").cloned());
        let title = album_data.get("title")?.as_str()?.to_string();

        // Process artists from album data
        let (artists, artist_seokeys, artist_ids) =
            if let Some(artist_array) = album_data.get("artist") {
                formatting::process_album_artists(&Some(artist_array.clone()))
            } else {
                // Fallback to first track's artist info if available
                if let Some(tracks) = response.get("tracks") {
                    if let Some(tracks_array) = tracks.as_array() {
                        if let Some(first_track) = tracks_array.first() {
                            if let Some(artist_array) = first_track.get("artist") {
                                formatting::process_album_artists(&Some(artist_array.clone()))
                            } else {
                                ("".to_string(), "".to_string(), "".to_string())
                            }
                        } else {
                            ("".to_string(), "".to_string(), "".to_string())
                        }
                    } else {
                        ("".to_string(), "".to_string(), "".to_string())
                    }
                } else {
                    ("".to_string(), "".to_string(), "".to_string())
                }
            };

        // Create images from artwork
        let images = if let Some(artwork) = album_data.get("artwork").and_then(|v| v.as_str()) {
            Some(Images::new(
                Some(artwork.replace("size_s.jpg", "size_l.jpg")),
                Some(artwork.to_string()),
                Some(artwork.replace("size_s.jpg", "size_m.jpg")),
            ))
        } else {
            None
        };

        // Process tracks if requested
        let tracks = if include_tracks {
            if let Some(tracks_value) = response.get("tracks") {
                if let Some(tracks_array) = tracks_value.as_array() {
                    let mut track_list = Vec::new();
                    for track_value in tracks_array {
                        if let Ok(track) = serde_json::from_value::<GaanaTrack>(track_value.clone())
                        {
                            if let Some(song) = self.process_gaana_track(&track) {
                                track_list.push(song);
                            }
                        }
                    }
                    if !track_list.is_empty() {
                        Some(track_list)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        Some(Album {
            seokey: seokey.clone(),
            album_id,
            title,
            artists,
            artist_seokeys,
            artist_ids,
            language: album_data
                .get("language")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            label: album_data
                .get("recordlevel")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            release_date: album_data
                .get("release_date")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            play_count: album_data
                .get("al_play_ct")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            favorite_count: formatting::extract_int(&album_data.get("favorite_count").cloned()),
            album_url: format!("https://gaana.com/album/{}", seokey),
            images,
            total_tracks: formatting::extract_int(&album_data.get("trackcount").cloned()),
            tracks,
        })
    }
    /// Process artist response from the API just like the Python version
    pub fn process_gaana_artist_response(
        &self,
        response: &Value,
        include_top_tracks: bool,
    ) -> Option<Artist> {
        // The Python code expects: result['artist'][0]
        let artist_array = response.get("artist")?.as_array()?;
        let artist_data = artist_array.first()?;

        let seokey = artist_data.get("seokey")?.as_str()?.to_string();
        let artist_id = formatting::extract_id(&artist_data.get("artist_id").cloned());
        let name = artist_data.get("name")?.as_str()?.to_string();

        // Create images from artwork
        let images = if let Some(artwork) = artist_data.get("atw").and_then(|v| v.as_str()) {
            Some(Images::new(
                Some(artwork.replace("size_m", "size_l")),
                Some(artwork.to_string()),
                Some(artwork.replace("size_m", "size_s")),
            ))
        } else {
            None
        };
        // Process top tracks if requested
        let top_tracks = if include_top_tracks {
            // For now, we don't include top tracks in the simple response
            // This would require an additional async call to get top tracks
            None
        } else {
            None
        };

        Some(Artist {
            seokey: seokey.clone(),
            artist_id,
            name,
            language: artist_data
                .get("language")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            play_count: artist_data
                .get("play_ct")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            favorite_count: formatting::extract_int(&artist_data.get("favorite_count").cloned()),
            artist_url: format!("https://gaana.com/artist/{}", seokey),
            images,
            top_tracks,
        })
    }
}
