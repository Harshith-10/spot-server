use serde_json::Value;

/// Extract ID from JSON value (could be string or number)
pub fn extract_id(value: &Option<Value>) -> String {
    match value {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => "0".to_string(),
    }
}

/// Extract integer from JSON value
pub fn extract_int(value: &Option<Value>) -> Option<i32> {
    match value {
        Some(Value::Number(n)) => n.as_i64().map(|i| i as i32),
        Some(Value::String(s)) => s.parse().ok(),
        _ => None,
    }
}

/// Process artist information from Gaana API response
pub fn process_artists(artist_value: &Option<Value>) -> (String, String, String, Option<String>) {
    match artist_value {
        Some(Value::Array(artists)) => {
            let mut names = Vec::new();
            let mut seokeys = Vec::new();
            let mut ids = Vec::new();
            let mut artist_image = None;

            for artist in artists {
                if let Some(name) = artist.get("name").and_then(|v| v.as_str()) {
                    names.push(name.to_string());
                }
                if let Some(seokey) = artist.get("seokey").and_then(|v| v.as_str()) {
                    seokeys.push(seokey.to_string());
                }
                if let Some(id) = artist.get("artist_id") {
                    ids.push(extract_id(&Some(id.clone())));
                }
                if artist_image.is_none() {
                    if let Some(artwork) = artist.get("artwork").and_then(|v| v.as_str()) {
                        artist_image = Some(artwork.to_string());
                    }
                }
            }

            (
                names.join(", "),
                seokeys.join(", "),
                ids.join(", "),
                artist_image,
            )
        }
        Some(Value::Object(artist)) => {
            let name = artist
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown Artist")
                .to_string();
            let seokey = artist
                .get("seokey")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let id = extract_id(&artist.get("artist_id").cloned());
            let artwork = artist
                .get("artwork")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            (name, seokey, id, artwork)
        }
        _ => (
            "Unknown Artist".to_string(),
            "".to_string(),
            "0".to_string(),
            None,
        ),
    }
}

/// Process album artist information (slightly different format)
pub fn process_album_artists(artist_value: &Option<Value>) -> (String, String, String) {
    let (names, seokeys, ids, _) = process_artists(artist_value);
    (names, seokeys, ids)
}

/// Process genre information
pub fn process_genres(genre_value: &Option<Value>) -> Option<String> {
    match genre_value {
        Some(Value::Array(genres)) => {
            let genre_names: Vec<String> = genres
                .iter()
                .filter_map(|g| g.get("name").and_then(|v| v.as_str()))
                .map(|s| s.to_string())
                .collect();

            if genre_names.is_empty() {
                None
            } else {
                Some(genre_names.join(", "))
            }
        }
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Object(genre)) => genre
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        _ => None,
    }
}

/// Validate and normalize language parameter
pub fn validate_language(lang: &str) -> String {
    let valid_languages = [
        "English",
        "Hindi",
        "Punjabi",
        "Telugu",
        "Tamil",
        "Bengali",
        "Gujarati",
        "Kannada",
        "Malayalam",
        "Marathi",
        "Odia",
        "Assamese",
    ];

    if valid_languages.contains(&lang) {
        lang.to_string()
    } else {
        "Hindi".to_string() // Default fallback
    }
}

/// Extract SEO key from Gaana URL
pub fn extract_seokey_from_url(url: &str) -> Option<String> {
    url.split('/').last().map(|s| s.to_string())
}

/// Limit results to specified count
pub fn limit_results<T>(results: Vec<T>, limit: Option<usize>) -> Vec<T> {
    let limit = limit.unwrap_or(10);
    results.into_iter().take(limit).collect()
}
