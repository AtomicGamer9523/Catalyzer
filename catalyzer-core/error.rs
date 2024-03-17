use core::fmt;

pub(crate) mod inner {
    use super::*;
    pub(crate) use std::io::ErrorKind as IoErrorKind;
    pub(crate) use std::io::Error as IoError;
    pub(crate) use std::string::FromUtf8Error;

    /// An Inner error type for Catalyzer operations.
    #[derive(Debug)]
    pub enum CatalyzerError {
        Io(IoError),
        Utf8(FromUtf8Error),
        RuntimeInitializationError,
        UnsupportedMethodError,
        NoAddress,
    }

    impl Clone for CatalyzerError {
        fn clone(&self) -> Self {
            match &self {
                Self::Io(e) => Self::Io(IoError::new(e.kind(), e.to_string())),
                Self::Utf8(e) => Self::Utf8(e.clone()),
                Self::RuntimeInitializationError => Self::RuntimeInitializationError,
                Self::UnsupportedMethodError => Self::UnsupportedMethodError,
                Self::NoAddress => Self::NoAddress,
            }
        }
    }
    
    impl std::error::Error for CatalyzerError {}

    impl fmt::Display for CatalyzerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Io(e) => write!(f, "An I/O error occurred: {}", e),
                Self::Utf8(_) => write!(f, "An error occurred while converting a byte array to a UTF-8 string"),
                Self::RuntimeInitializationError => write!(f, "An error occurred while initializing the runtime"),
                Self::UnsupportedMethodError => write!(f, "The provided method is not supported"),
                Self::NoAddress => write!(f, "No address was provided"),
            }
        }
    }
}

pub(crate) use inner::{CatalyzerError as Inner, *};

/// An error type for Catalyzer operations.
/// 
/// This type is a wrapper around various error types that can occur during
/// Catalyzer operations.
/// 
/// For ease of use, you can always use the `?` operator to propagate errors.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CatalyzerError(Inner);

#[allow(non_upper_case_globals)]
impl CatalyzerError {
    pub const RuntimeInitializationError: Self = Self(Inner::RuntimeInitializationError);
    pub const UnsupportedMethodError: Self = Self(Inner::UnsupportedMethodError);
    pub const NoAddress: Self = Self(Inner::NoAddress);
    /// Creates a new `CatalyzerError` from the given inner error.
    #[inline]
    pub const fn new(inner: Inner) -> Self {
        Self(inner)
    }
    /// Returns the inner error.
    #[inline]
    pub fn into_inner(self) -> Inner {
        self.0
    }
}

impl std::error::Error for CatalyzerError {}

impl fmt::Display for CatalyzerError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<IoError> for CatalyzerError {
    #[inline]
    fn from(e: IoError) -> Self {
        Self(Inner::Io(e))
    }
}

impl From<FromUtf8Error> for CatalyzerError {
    #[inline]
    fn from(e: FromUtf8Error) -> Self {
        Self(Inner::Utf8(e))
    }
}

use core::result::Result as R;
/// A specialized `Result` type for Catalyzer operations.
pub type Result<T = (), E = CatalyzerError> = R<T, E>;

impl ::axum::response::IntoResponse for CatalyzerError {
    fn into_response(self) -> axum::response::Response {
        let v = axum::http::Response::builder()
            .status(::axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(self.to_string().into());
        #[cfg(debug_assertions)]
        { v.unwrap_or_default() }
        #[cfg(not(debug_assertions))]
        unsafe { v.unwrap_unchecked() }
    }
}
