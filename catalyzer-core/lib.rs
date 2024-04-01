//! Catalyzer Core

#![crate_name = "catalyzer"]

mod error;
mod app;

#[doc(hidden)]
#[path = "internals/runtime.rs"]
pub(crate) mod runtime;

#[path = "request/mod.rs"]
pub mod req;
#[path = "response/mod.rs"]
pub mod res;

pub mod internals;
#[doc(inline)]
pub use error::{Result, CatalyzerError};
#[doc(inline)]
pub use app::App;
