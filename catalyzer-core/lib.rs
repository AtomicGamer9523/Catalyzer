//! Catalyzer Core

mod error;
mod app;

#[path = "request/mod.rs"]
pub mod req;
#[path = "response/mod.rs"]
pub mod res;

pub use self::error::{Result, CatalyzerError};
pub use app::App;

pub mod http {
    pub use ::core::future::{Future, IntoFuture};
    pub use ::axum::serve::IncomingStream;
    pub use ::axum::Router;
    pub use ::tower::Service;
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

// #[doc(hidden)]
pub mod __internals__ {
    pub use ::axum::response::IntoResponse as AxumIntoResponse;
    pub use ::axum::handler::Handler as AxumHandler;

    pub use crate::error::inner::CatalyzerHTTPError as InnerCatalyzerHTTPError;
    pub use crate::error::inner::CatalyzerError as InnerCatalyzerError;
    pub use crate::app::launch::Launch as LaunchedApp;

    #[path = "../runtime.rs"]
    pub mod runtime;
}
