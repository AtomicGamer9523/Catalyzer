//! **!!! INTERNALS !!! DO NOT USE DIRECTLY!!!**

/// Runtime management for the Catalyzer framework.
pub mod runtime {
    pub use crate::runtime::*;
}
mod handlers;

pub use crate::internals::handlers::{HandlerMetadata, AxumHandler, Method, CatalyzerService, TowerService};
pub use crate::internals::runtime::{CatalyzerRuntimeBuilder, CatalyzerRuntime};
pub use crate::error::inner::CatalyzerIoError as InnerCatalyzerIoError;
pub use crate::error::inner::CatalyzerError as InnerCatalyzerError;
pub use crate::app::launch::CatalyzedApp;
pub use ::axum::Router as AxumRouter;
pub use ::utils::*;

pub mod content_loader;

/// Re-exports of crates used in Catalyzer.
pub mod crates {
    pub use ::axum;
    pub use ::tower;
    pub use ::tokio;
}
