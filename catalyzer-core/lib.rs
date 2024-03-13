//! Catalyzer Core

mod error;
mod app;

/// A collection of request types.
#[path = "request/mod.rs"]
pub mod req;
/// A collection of response types.
#[path = "response/mod.rs"]
pub mod res;

pub use self::error::{Result, CatalyzerError};
pub use app::App;

/// HTTP types.
/// 
/// As well as some other goodies.
pub mod http {
    pub use ::core::future::{Future, IntoFuture};
    #[doc(hidden)]
    pub use ::core::pin::Pin;
    pub use ::axum::serve::IncomingStream;
    pub use ::axum::body::Body;
    pub use ::tower::Service;
    pub use crate::app::service::CatalyzerService;
    pub use ::axum::http::{
        Extensions,
        header, HeaderMap, HeaderName, HeaderValue,
        method, Method,
        request, response, Request, Response,
        status, StatusCode,
        uri, Uri,
        version, Version
    };
}

/// This module contains the "public" internals of the Catalyzer library.
/// 
/// They are not meant to be used directly, therefore they are **NOT** guaranteed to be stable.
/// 
/// Use at your own risk.
// #[doc(hidden)]
pub mod __internals__ {
    pub use ::axum::response::IntoResponse as AxumIntoResponse;
    pub use ::axum::handler::Handler as AxumHandler;
    pub use ::axum::routing::Router as AxumRouter;
    pub use ::axum::routing as axum_routing;

    pub use crate::error::inner::CatalyzerHTTPError as InnerCatalyzerHTTPError;
    pub use crate::error::inner::CatalyzerError as InnerCatalyzerError;
    pub use crate::app::launch::{BoundApp, CatalyzedApp};

    #[path = "../runtime.rs"]
    pub mod runtime;
}
