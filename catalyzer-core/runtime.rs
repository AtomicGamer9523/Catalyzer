use ::core::future::Future;
use crate::*;

pub struct Runtime {
    pub tokio_runtime: ::tokio::runtime::Runtime,
}

impl Runtime {
    pub fn init() -> Result<Self> {
        let tokio_runtime = ::tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        Ok(Self { tokio_runtime })
    }
    pub fn run_main<F>(&self, f: F) -> Result where
        F: Future<Output = Result> + Send + 'static,
    {
        self.tokio_runtime.block_on(f)
    }
}
