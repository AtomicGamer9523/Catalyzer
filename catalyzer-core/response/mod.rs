//! A collection of response types and utilities.

/// A trait for types that can be converted into a [raw response].
/// 
/// [raw response]: type.RawResponse.html
pub trait IntoRawResponse: ::axum::response::IntoResponse {}
impl<T: ::axum::response::IntoResponse> IntoRawResponse for T {}

use axum::body::Body;

/// A type representing a raw response.
pub type RawResponse<T = Body> = ::axum::response::Response<T>;

mod html;
pub use html::Html;
