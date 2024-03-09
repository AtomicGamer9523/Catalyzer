use std::io::Error as IoError;
use std::string::FromUtf8Error;
use core::fmt;

pub(crate) mod inner {
    pub use ::core::result::Result as R;
    use axum::http::*;
    use super::*;

    #[derive(Debug)]
    pub enum CatalyzerError {
        Io(IoError),
        Utf8(FromUtf8Error),
        Http(CatalyzerHTTPError),
        NoAddress,
        AddressParseError,
        #[cfg(feature = "html")]
        HtmlValidationError(::html::HtmlError),
    }

    impl fmt::Display for CatalyzerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Io(e) => write!(f, "IO error: {}", e),
                Self::Utf8(e) => write!(f, "UTF-8 error: {}", e),
                Self::Http(e) => write!(f, "HTTP error: {}", e),
                Self::NoAddress => write!(f, "No address provided"),
                Self::AddressParseError => write!(f, "Failed to parse address"),
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

    #[derive(Debug)]
    pub enum CatalyzerHTTPError {
        StatusCode(status::InvalidStatusCode),
        Method(method::InvalidMethod),
        Uri(uri::InvalidUri),
        UriParts(uri::InvalidUriParts),
        HeaderName(header::InvalidHeaderName),
        HeaderValue(header::InvalidHeaderValue)
    }
    
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

#[repr(transparent)]
pub struct CatalyzerError(Inner);

impl std::error::Error for CatalyzerError {}

impl CatalyzerError {
    pub const NO_ADDRESS: Self = Self(Inner::NoAddress);
    pub const ADDRESS_PARSE_ERROR: Self = Self(Inner::AddressParseError);
    pub const INVALID_STATUS_CODE: Self = Self(Inner::INVALID_STATUS_CODE);
    pub const INVALID_METHOD: Self = Self(Inner::INVALID_METHOD);
    pub const INVALID_HEADER_NAME: Self = Self(Inner::INVALID_HEADER_NAME);
    pub const INVALID_HEADER_VALUE: Self = Self(Inner::INVALID_HEADER_VALUE);
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

pub type Result<T = (), E = CatalyzerError> = R<T, E>;

impl ::axum::response::IntoResponse for CatalyzerError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
