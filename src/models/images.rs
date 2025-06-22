use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Images {
    pub large_artwork: Option<String>,
    pub medium_artwork: Option<String>,
    pub small_artwork: Option<String>,
}

impl Images {
    pub fn new(large: Option<String>, medium: Option<String>, small: Option<String>) -> Self {
        Self {
            large_artwork: large,
            medium_artwork: medium,
            small_artwork: small,
        }
    }
}
