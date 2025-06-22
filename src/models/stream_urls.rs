use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StreamUrls {
    pub very_high_quality: Option<String>, // 320kbps
    pub high_quality: Option<String>,      // 128kbps
    pub medium_quality: Option<String>,    // 64kbps
    pub low_quality: Option<String>        // 16kbps
}

impl StreamUrls {
    pub fn new(
        vhq: Option<String>,
        hq: Option<String>,
        mq: Option<String>,
        lq: Option<String>,
    ) -> Self {
        Self {
            very_high_quality: vhq,
            high_quality: hq,
            medium_quality: mq,
            low_quality: lq
        }
    }
}
