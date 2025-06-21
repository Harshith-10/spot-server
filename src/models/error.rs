use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}

impl ApiError {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
        }
    }

    pub fn not_found(query: &str) -> Self {
        Self::new(
            "No results found",
            &format!("No search results for the given query: {}", query),
        )
    }

    pub fn invalid_seokey(seokey: &str) -> Self {
        Self::new(
            "Invalid seokey",
            &format!("The provided seokey is invalid or not found: {}", seokey),
        )
    }

    pub fn internal_error(message: &str) -> Self {
        Self::new("Internal server error", message)
    }

    pub fn invalid_parameter(param: &str, message: &str) -> Self {
        Self::new(&format!("Invalid parameter: {}", param), message)
    }
}
