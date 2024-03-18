//! Macros for the Catalyzer framework.

#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro2::TokenStream as T;
use proc_macro::TokenStream;

mod main_func;
mod routes;
mod app;

/// Marks a function as the main entry point for the application.
/// 
/// # Example
/// 
/// ```rust
/// # use catalyzer::*;
/// #[main]
/// fn main() {
///     // Your code here (can be both sync and async)
/// }
/// ```
#[proc_macro_attribute]
pub fn main(cfg: TokenStream, input: TokenStream) -> TokenStream {
    main_func::main(cfg.into(), input.into()).into()
}

/// A shortcut for creating an [`App`] instance.
/// 
/// [`App`]: struct.App.html
/// 
/// # Example
/// 
/// ```rust
/// # use catalyzer::*;
/// #[main]
/// fn main() {
/// App![index]
///     .bind("0.0.0.0:3000")?
///     .launch()
/// }
/// 
/// #[get("/")]
/// fn index() {
///     "Hello, world!"
/// }
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn App(input: TokenStream) -> TokenStream {
    app::app(input.into()).into()
}

macro_rules! routes {
    ($(
        $(#[$attr:meta])*
        @$method:ident
    )+) => ($(
        $(#[$attr])*
        #[proc_macro_attribute]
        pub fn $method(cfg: TokenStream, input: TokenStream) -> TokenStream {
            routes::$method(cfg.into(), input.into()).into()
        }
    )+)
}

routes!(
    /// A route handler for the `GET` method.
    @get
    /// A route handler for the `POST` method.
    @post
    /// A route handler for the `PUT` method.
    @put
    /// A route handler for the `DELETE` method.
    @delete
    /// A route handler for the `PATCH` method.
    @patch
    /// A route handler for the `HEAD` method.
    @head
    /// A route handler for the `OPTIONS` method.
    @options
    /// A route handler for the `TRACE` method.
    @trace
);
