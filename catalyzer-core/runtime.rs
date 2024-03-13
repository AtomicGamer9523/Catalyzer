//! Runtime module for the Catalyzer core library.

use ::core::future::Future;
use crate::*;

/// The runtime for the Catalyzer core library.
#[derive(Debug)]
pub struct Runtime {
    /// The Tokio runtime used by the runtime.
    pub tokio_runtime: ::tokio::runtime::Runtime,
}

impl Runtime {
    /// Initialize the runtime.
    pub fn init() -> Result<Self> {
        let tokio_runtime = ::tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        Ok(Self { tokio_runtime })
    }
    /// Runs the main function of the application.
    pub fn run_main<F>(&self, f: F) -> Result where
        F: Future<Output = Result> + Send + 'static,
    {
        ::logger::SimpleLogger::new()
            .env()
            .with_level(::log::LevelFilter::Trace)
            .init()
            .map_err(|_| CatalyzerError::RUNTIME_INIT_ERROR)?;
        self.tokio_runtime.block_on(f)
    }
}
