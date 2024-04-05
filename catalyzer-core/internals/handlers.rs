pub use ::axum::handler::Handler as AxumHandler;
pub use ::tower::Service as TowerService;
pub use ::axum::http::Method;

use core::convert::Infallible;
use crate::req::RawRequest;

/// A trait that represents a handler metadata.
/// 
/// They are used to store the metadata of a handler,
/// and are automatically implemented by the `#[get]`, `#[post]`, etc. macros.
/// 
/// This trait is used internally by Catalyzer, and should not be implemented manually.
/// 
/// With `#[get]`:
/// 
/// ```rust
/// # use catalyzer::*;
/// #[get("/")]
/// fn index() {
///     "Hello, world!"
/// }
/// ```
/// 
/// Manual implementation:
/// 
/// ```rust
/// # use catalyzer::*;
/// async fn index() -> impl ::catalyzer::res::IntoRawResponse {
///     "Hello, world!"
/// }
/// #[doc(hidden)]
/// #[repr(transparent)]
/// #[allow(non_camel_case_types)]
/// struct index_metadata;
/// impl ::catalyzer::internals::HandlerMetadata for index_metadata {
///     const PATH: &'static str = "/";
///     const METHOD: ::catalyzer::internals::Method = ::catalyzer::internals::Method::GET;
/// }
/// ```
pub trait HandlerMetadata {
    /// Path to mount the handler on.
    const PATH: &'static str;
    /// Method to handle.
    const METHOD: Method;
}

/// A trait that represents a Catalyzer service.
/// 
/// All services must implement this trait (and the [`TowerService`] trait).
/// 
/// [`TowerService`]: https://docs.rs/tower/0.4.4/tower/trait.Service.html
pub trait CatalyzerService: TowerService<RawRequest, Error = Infallible> {
    /// Path to mount the service on.
    const PATH: &'static str;
}
