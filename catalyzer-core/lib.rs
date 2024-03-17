//! Catalyzer Core

#![crate_name = "catalyzer"]

mod error;
mod app;

#[path = "internals/runtime.rs"]
pub(crate) mod runtime;

#[path = "request/mod.rs"]
pub mod req;
#[path = "response/mod.rs"]
pub mod res;

#[path = "internals/mod.rs"]
pub mod __internals__;

pub use error::{Result, CatalyzerError};
pub use app::App;
