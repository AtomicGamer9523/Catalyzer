use std::io::Error as IoError;
use std::string::FromUtf8Error;
use core::fmt;

pub(crate) mod inner {
    pub use ::core::result::Result as R;
    use axum::http::*;
    use super::*;

    /// The Inner error type for Catalyzer operations.
    #[derive(Debug)]
    pub enum CatalyzerError {
        /// An IO error.
        Io(IoError),
        /// A UTF-8 error.
        Utf8(FromUtf8Error),
        /// An HTTP error.
        Http(CatalyzerHTTPError),
        /// A basic error indicating that no address was provided.
        NoAddress,
        /// A basic error indicating that the address provided could not be parsed.
        AddressParseError,
        /// A basic error indicating that the runtime failed to initialize.
        RuntimeInitializationError,
        /// A basic error indicating that the HTML provided was invalid.
        #[cfg(feature = "html")]
        HtmlValidationError(::html::HtmlError),
    }

    impl Clone for CatalyzerError {
        #[inline]
        fn clone(&self) -> Self {
            match self {
                Self::Io(e) => Self::Io(IoError::new(e.kind(), e.to_string())),
                Self::Utf8(e) => Self::Utf8(e.clone()),
                Self::Http(e) => Self::Http(e.clone()),
                Self::NoAddress => Self::NoAddress,
                Self::AddressParseError => Self::AddressParseError,
                Self::RuntimeInitializationError => Self::RuntimeInitializationError,
                #[cfg(feature = "html")]
                Self::HtmlValidationError(e) => Self::HtmlValidationError(e.clone()),
            }
        }
    }

    impl fmt::Display for CatalyzerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Io(e) => write!(f, "IO error: {}", e),
                Self::Utf8(e) => write!(f, "UTF-8 error: {}", e),
                Self::Http(e) => write!(f, "HTTP error: {}", e),
                Self::NoAddress => write!(f, "No address provided"),
                Self::AddressParseError => write!(f, "Failed to parse address"),
                Self::RuntimeInitializationError => write!(f, "Failed to initialize runtime"),
                #[cfg(feature = "html")]
                Self::HtmlValidationError(e) => write!(f, "HTML validation error: {}", e),
            }
        }
    }

    impl std::error::Error for CatalyzerError {}

    macro_rules! gen {
        ($name: ident: $type:ty) => (
            pub(crate) const $name: $type = {
                struct ZeroByte;
                unsafe { core::mem::transmute(ZeroByte) }
            };
        )
    }

    gen!(STATUS_CODE: status::InvalidStatusCode);
    gen!(METHOD: method::InvalidMethod);
    gen!(HEADER_NAME: header::InvalidHeaderName);
    gen!(HEADER_VALUE: header::InvalidHeaderValue);

    /// The Inner error type for HTTP errors.
    #[derive(Debug)]
    pub enum CatalyzerHTTPError {
        /// An invalid status code.
        StatusCode(status::InvalidStatusCode),
        /// An invalid method.
        Method(method::InvalidMethod),
        /// An invalid URI.
        Uri(uri::InvalidUri),
        /// Invalid URI parts.
        UriParts(uri::InvalidUriParts),
        /// An invalid header name.
        HeaderName(header::InvalidHeaderName),
        /// An invalid header value.
        HeaderValue(header::InvalidHeaderValue),
    }

    impl Clone for CatalyzerHTTPError {
        #[inline]
        fn clone(&self) -> Self {
            match self {
                Self::StatusCode(_) => Self::StatusCode(STATUS_CODE),
                Self::Method(_) => Self::Method(METHOD),
                Self::Uri(e) => Self::Uri(bit_clone(&e)),
                Self::UriParts(e) => Self::UriParts(bit_clone(&e)),
                Self::HeaderName(_) => Self::HeaderName(HEADER_NAME),
                Self::HeaderValue(_) => Self::HeaderValue(HEADER_VALUE),
            }
        }
    }

    fn bit_clone<T>(ptr: &T) -> T {
        unsafe {
            let layout = std::alloc::Layout::new::<T>();
            let new = std::alloc::alloc(layout);
            if new.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            core::ptr::copy_nonoverlapping(ptr, new as *mut T, 1);
            core::ptr::read(new as *const T)
        }
    }

    #[doc(hidden)]
    impl CatalyzerError {
        pub const INVALID_STATUS_CODE: Self = Self::Http(CatalyzerHTTPError::StatusCode(STATUS_CODE));
        pub const INVALID_METHOD: Self = Self::Http(CatalyzerHTTPError::Method(METHOD));
        pub const INVALID_HEADER_NAME: Self = Self::Http(CatalyzerHTTPError::HeaderName(HEADER_NAME));
        pub const INVALID_HEADER_VALUE: Self = Self::Http(CatalyzerHTTPError::HeaderValue(HEADER_VALUE));
    }

    impl From<status::InvalidStatusCode> for CatalyzerHTTPError {
        #[inline]
        fn from(e: status::InvalidStatusCode) -> Self {
            Self::StatusCode(e)
        }
    }

    impl From<method::InvalidMethod> for CatalyzerHTTPError {
        #[inline]
        fn from(e: method::InvalidMethod) -> Self {
            Self::Method(e)
        }
    }

    impl From<uri::InvalidUri> for CatalyzerHTTPError {
        #[inline]
        fn from(e: uri::InvalidUri) -> Self {
            Self::Uri(e)
        }
    }

    impl From<uri::InvalidUriParts> for CatalyzerHTTPError {
        #[inline]
        fn from(e: uri::InvalidUriParts) -> Self {
            Self::UriParts(e)
        }
    }

    impl From<header::InvalidHeaderName> for CatalyzerHTTPError {
        #[inline]
        fn from(e: header::InvalidHeaderName) -> Self {
            Self::HeaderName(e)
        }
    }

    impl From<header::InvalidHeaderValue> for CatalyzerHTTPError {
        #[inline]
        fn from(e: header::InvalidHeaderValue) -> Self {
            Self::HeaderValue(e)
        }
    }

    impl fmt::Display for CatalyzerHTTPError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::StatusCode(e) => write!(f, "Invalid status code: {}", e),
                Self::Method(e) => write!(f, "Invalid method: {}", e),
                Self::Uri(e) => write!(f, "Invalid URI: {}", e),
                Self::UriParts(e) => write!(f, "Invalid URI parts: {}", e),
                Self::HeaderName(e) => write!(f, "Invalid header name: {}", e),
                Self::HeaderValue(e) => write!(f, "Invalid header value: {}", e),
            }
        }
    }

    impl std::error::Error for CatalyzerHTTPError {}
}

use inner::CatalyzerError as Inner;
use inner::CatalyzerHTTPError;
use inner::R;

/// An error type for Catalyzer operations.
/// 
/// This type is a wrapper around various error types that can occur during
/// Catalyzer operations.
/// 
/// For ease of use, you can always use the `?` operator to propagate errors.
#[derive(Clone)]
#[repr(transparent)]
pub struct CatalyzerError(Inner);

impl std::error::Error for CatalyzerError {}

impl CatalyzerError {
    /// A basic error indicating that no address was provided.
    pub const NO_ADDRESS: Self = Self(Inner::NoAddress);
    /// A basic error indicating that the address provided could not be parsed.
    pub const ADDRESS_PARSE_ERROR: Self = Self(Inner::AddressParseError);
    /// A basic error indicating that the status code provided was invalid.
    pub const INVALID_STATUS_CODE: Self = Self(Inner::INVALID_STATUS_CODE);
    /// A basic error indicating that the method provided was invalid.
    pub const INVALID_METHOD: Self = Self(Inner::INVALID_METHOD);
    /// A basic error indicating that the header name provided was invalid.
    pub const INVALID_HEADER_NAME: Self = Self(Inner::INVALID_HEADER_NAME);
    /// A basic error indicating that the header value provided was invalid.
    pub const INVALID_HEADER_VALUE: Self = Self(Inner::INVALID_HEADER_VALUE);
    /// A basic error indicating that the runtime failed to initialize.
    pub const RUNTIME_INIT_ERROR: Self = Self(Inner::RuntimeInitializationError);
    /// Returns the inner error.
    #[inline]
    pub fn into_inner(self) -> Inner {
        self.0
    }
}

impl fmt::Debug for CatalyzerError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

impl From<CatalyzerHTTPError> for CatalyzerError {
    #[inline]
    fn from(e: CatalyzerHTTPError) -> Self {
        Self(Inner::Http(e))
    }
}

impl From<::axum::http::status::InvalidStatusCode> for CatalyzerError {
    #[inline]
    fn from(e: ::axum::http::status::InvalidStatusCode) -> Self {
        Self(Inner::Http(CatalyzerHTTPError::StatusCode(e)))
    }
}

impl From<::axum::http::method::InvalidMethod> for CatalyzerError {
    #[inline]
    fn from(e: ::axum::http::method::InvalidMethod) -> Self {
        Self(Inner::Http(CatalyzerHTTPError::Method(e)))
    }
}

impl From<::axum::http::uri::InvalidUri> for CatalyzerError {
    #[inline]
    fn from(e: ::axum::http::uri::InvalidUri) -> Self {
        Self(Inner::Http(CatalyzerHTTPError::Uri(e)))
    }
}

impl From<::axum::http::uri::InvalidUriParts> for CatalyzerError {
    #[inline]
    fn from(e: ::axum::http::uri::InvalidUriParts) -> Self {
        Self(Inner::Http(CatalyzerHTTPError::UriParts(e)))
    }
}

impl From<::axum::http::header::InvalidHeaderName> for CatalyzerError {
    #[inline]
    fn from(e: ::axum::http::header::InvalidHeaderName) -> Self {
        Self(Inner::Http(CatalyzerHTTPError::HeaderName(e)))
    }
}

impl From<::axum::http::header::InvalidHeaderValue> for CatalyzerError {
    #[inline]
    fn from(e: ::axum::http::header::InvalidHeaderValue) -> Self {
        Self(Inner::Http(CatalyzerHTTPError::HeaderValue(e)))
    }
}

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
