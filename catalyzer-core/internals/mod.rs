//! **!!! INTERNALS !!! DO NOT USE DIRECTLY!!!**

mod handlers;

pub use handlers::{HandlerMetadata, AxumHandler, Method};
pub use axum::Router as AxumRouter;
pub use crate::runtime::{CatalyzerRuntime, CatalyzerRuntimeBuilder};
pub use crate::app::launch::CatalyzedApp;
pub use crate::error::inner::CatalyzerError as InnerCatalyzerError;
pub mod content_loader;
pub mod runtime;

/// Re-exports of crates used in Catalyzer.
pub mod crates {
    pub use ::axum;
    pub use ::tower;
    pub use ::tokio;
}
