//! Catalyzer HTML integration

use ::axum::response::IntoResponse;
use ::std::path::Path;
use ::core::fmt;
use crate::Result;

#[repr(transparent)]
pub struct Html(InnerHtml);

#[doc(hidden)]
pub enum InnerHtml {
    String(String),
    // DOMTree(::axohtml::dom::DOMTree<String>),
}

impl fmt::Display for InnerHtml {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            // Self::DOMTree(tree) => tree.fmt(f),
        }
    }
}

impl fmt::Display for Html {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Html {
    pub async fn from_file<P>(path: P) -> Result<Html> where
        P: AsRef<Path>,
    {
        utils::ResultTransformer::auto(
            tokio::fs::read_to_string(path).await
        )
    }
}

impl From<&str> for Html {
    #[inline]
    fn from(this: &str) -> Html {
        Self(InnerHtml::String(this.to_string()))
    }
}

impl From<String> for Html {
    #[inline]
    fn from(this: String) -> Html {
        Self(InnerHtml::String(this))
    }
}

impl From<Html> for String {
    #[inline]
    fn from(this: Html) -> String {
        this.to_string()
    }
}

impl IntoResponse for Html {
    fn into_response(self) -> ::axum::response::Response<::axum::body::Body> {
        ::axum::response::Response::builder()
            .header(::axum::http::header::CONTENT_TYPE, "text/html")
            .body(self.to_string().into())
            .unwrap_or_default()
    }
}

#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use macros::html_macro;

pub mod prelude {
    
}
