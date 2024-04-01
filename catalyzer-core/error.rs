use core::{fmt, cmp};
pub(crate) mod inner {
    pub(crate) use std::io::ErrorKind as IoErrorKind;
    pub(crate) use std::string::FromUtf8Error;
    pub(crate) use std::io::Error as IoError;
    use super::*;

    /// A cloneable, transparent wrapper around an I/O error.
    #[repr(transparent)]
    pub struct CatalyzerIoError(pub(super) IoError);
    impl CatalyzerIoError {
        /// Returns the kind of the error.
        #[inline]
        pub fn kind(&self) -> IoErrorKind {
            self.0.kind()
        }
    }
    impl Clone for CatalyzerIoError {
        #[inline]
        fn clone(&self) -> Self {
            Self(IoError::new(self.0.kind(), self.0.to_string()))
        }
    }
    impl fmt::Debug for CatalyzerIoError {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }
    impl fmt::Display for CatalyzerIoError {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }
    impl cmp::PartialEq for CatalyzerIoError {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.0.kind().eq(&other.0.kind())
        }
    }
    impl cmp::Eq for CatalyzerIoError {}
    impl From<IoError> for CatalyzerIoError {
        #[inline]
        fn from(e: IoError) -> Self {
            Self(e)
        }
    }
    impl From<CatalyzerIoError> for IoError {
        #[inline]
        fn from(e: CatalyzerIoError) -> Self {
            e.0
        }
    }
    impl std::error::Error for CatalyzerIoError {}

    /// An Inner error type for Catalyzer operations.
    #[derive(Debug, Clone)]
    pub enum CatalyzerError {
        /// An I/O error occurred.
        Io(CatalyzerIoError),
        /// An error occurred while converting a byte array to a UTF-8 string.
        Utf8(FromUtf8Error),
        /// An error occurred while initializing the runtime.
        RuntimeInitializationError,
        /// The provided method is not supported.
        UnsupportedMethodError,
        /// No address was provided.
        NoAddress,
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
    /// A shortcut for creating a [`RuntimeInitializationError`].
    /// 
    /// [`RuntimeInitializationError`]: crate::internals::InnerCatalyzerError::RuntimeInitializationError
    pub const RuntimeInitializationError: Self = Self(Inner::RuntimeInitializationError);
    /// A shortcut for creating a [`UnsupportedMethodError`].
    /// 
    /// [`UnsupportedMethodError`]: crate::internals::InnerCatalyzerError::UnsupportedMethodError
    pub const UnsupportedMethodError: Self = Self(Inner::UnsupportedMethodError);
    /// A shortcut for creating a [`NoAddress`] error.
    /// 
    /// [`NoAddress`]: crate::internals::InnerCatalyzerError::NoAddress
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

impl From<CatalyzerIoError> for CatalyzerError {
    #[inline]
    fn from(e: CatalyzerIoError) -> Self {
        Self(Inner::Io(e))
    }
}

impl From<IoError> for CatalyzerError {
    #[inline]
    fn from(e: IoError) -> Self {
        Self(Inner::Io(CatalyzerIoError(e)))
    }
}

impl From<FromUtf8Error> for CatalyzerError {
    #[inline]
    fn from(e: FromUtf8Error) -> Self {
        Self(Inner::Utf8(e))
    }
}

impl From<Inner> for CatalyzerError {
    #[inline]
    fn from(e: Inner) -> Self {
        Self(e)
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
