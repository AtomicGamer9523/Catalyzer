//! A collection of request types and utilities.

use axum::body::Body;

/// A type representing a raw request.
pub type RawRequest<T = Body> = ::axum::extract::Request<T>;

mod state;
pub use state::State;
