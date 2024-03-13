#![cfg(feature = "automatic-defaults")]

use super::*;

pub(super) fn make_default_router<S>() -> AxumRouter<S> where
    S: Clone + Send + Sync + 'static,
{
    const DEFAULT_HTML: &str = include_str!("default.html");
    const NOT_FOUND_HTML: &str = include_str!("404.html");
    const LOGO192_ICO: &[u8] = include_bytes!("../../../doc/logo192.ico");
    const LOGO32_ICO: &[u8] = include_bytes!("../../../doc/logo32.ico");
    async fn index() -> RawResponse {
        RawResponse::builder()
            .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .header(axum::http::header::CACHE_CONTROL, "public, max-age=31536000")
            .body(DEFAULT_HTML.into())
            .unwrap_or_default()
    }
    async fn not_found() -> RawResponse {
        RawResponse::builder()
            .status(404)
            .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .header(axum::http::header::CACHE_CONTROL, "public, max-age=31536000")
            .body(NOT_FOUND_HTML.into())
            .unwrap_or_default()
    }
    fn get_favicon(query: Option<axum::extract::RawQuery>) -> &'static [u8] {
        let query = match query {
            None => return LOGO32_ICO,
            Some(query) => query,
        };
        let query = match query.0 {
            None => return LOGO32_ICO,
            Some(query) => query,
        };
        if query == "s=192" {
            return LOGO192_ICO;
        }
        LOGO32_ICO
    }
    async fn favicon(query: Option<axum::extract::RawQuery>) -> RawResponse {
        RawResponse::builder()
            .header(axum::http::header::CONTENT_TYPE, "image/x-icon")
            .header(axum::http::header::CACHE_CONTROL, "public, max-age=31536000")
            .body(get_favicon(query).into())
            .unwrap_or_default()
    }
    AxumRouter::<S>::new()
        .route("/", axum::routing::get(index))
        .route("/favicon.ico", axum::routing::get(favicon))
        .fallback(axum::routing::get(not_found))
}
