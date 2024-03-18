//! A collection of response types and utilities.

use ::axum::response::{IntoResponse as Ir, Response as R};
use crate::__internals__::content_loader::*;
use axum::body::Body;
use std::path::Path;
use core::fmt;
use crate::*;
mod builtins;

/// A trait for types that can be converted into a [raw response].
/// 
/// [raw response]: type.RawResponse.html
pub trait IntoRawResponse: Ir {}
impl<R: Ir> IntoRawResponse for R {}

/// A type representing a raw response.
pub type RawResponse<T = Body> = R<T>;

pub use builtins::*;
