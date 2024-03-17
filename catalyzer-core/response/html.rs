use axum::body::Body;
use utils::ResultTransformer;
use std::path::Path;

use res::RawResponse;
use crate::*;

#[derive(Debug)]
pub struct Html {
    data: String,
}

impl Html {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        tokio::fs::read_to_string(path).await.map_auto()
    }
}

impl From<String> for Html {
    fn from(s: String) -> Self {
        Self {
            data: s,
        }
    }
}

impl core::fmt::Display for Html {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.data.fmt(f)
    }
}

impl From<Html> for Body {
    fn from(html: Html) -> Self {
        Body::from(html.to_string())
    }
}

impl crate::res::IntoRawResponse for Html {
    fn into_response(self) -> RawResponse {
        RawResponse::builder()
            .header("Content-Type", "text/html")
            .body(self.into())
            .unwrap_or_default()
    }
}
