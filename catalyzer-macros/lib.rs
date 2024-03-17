//! Macros for the Catalyzer framework.

#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro2::TokenStream as T;
use proc_macro::TokenStream;

mod main_func;
mod routes;
mod app;

#[proc_macro_attribute]
pub fn main(cfg: TokenStream, input: TokenStream) -> TokenStream {
    main_func::main(cfg.into(), input.into()).into()
}

#[proc_macro]
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
