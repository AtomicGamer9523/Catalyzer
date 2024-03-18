pub use ::axum::handler::Handler as AxumHandler;
pub use ::axum::http::Method;

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
/// impl ::catalyzer::__internals__::HandlerMetadata for index_metadata {
///     const PATH: &'static str = "/";
///     const METHOD: ::catalyzer::__internals__::Method = ::catalyzer::__internals__::Method::GET;
/// }
/// ```
pub trait HandlerMetadata {
    /// Path to mount the handler on.
    const PATH: &'static str;
    /// Method to handle.
    const METHOD: Method;
}
