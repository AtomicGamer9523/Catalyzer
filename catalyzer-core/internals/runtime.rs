pub(crate) use tokio::runtime::Runtime as TokioRuntime;
use tokio::runtime::Builder as TokioRuntimeBuilder;
use core::future::Future;
use utils::*;
use crate::*;

/// A runtime for the Catalyzer framework.
/// 
/// You most likely won't need to use this directly,
/// as everything is handled by the `#[main]` macro.
#[derive(Debug)]
pub struct CatalyzerRuntime {
    tokio: TokioRuntime
}

/// A builder for the [`CatalyzerRuntime`](crate::internals::runtime::CatalyzerRuntime).
#[derive(Debug)]
pub struct CatalyzerRuntimeBuilder {
    tokio: Option<TokioRuntime>,
}

impl CatalyzerRuntime {
    fn default_preinit() -> Result<CatalyzerRuntime> {
        #[cfg(feature = "builtin-logger")]
        {
            let log_level = std::env::var("CATALYZER_LOG_LEVEL").unwrap_or("info".to_string());
            let log_level = log_level.parse().unwrap_or(log::LevelFilter::Info);
            let mut l = ::builtin_logger::SimpleLogger::new()
                .with_level(log_level);
            #[cfg(debug_assertions)]
            { l = l.with_colors(true); }
            #[cfg(not(debug_assertions))]
            { l = l.with_colors(false); }
            let _ = l.init();
        }
        use std::sync::atomic::{AtomicU8, Ordering};
        static ATOMIC_ID: AtomicU8 = AtomicU8::new(0);
        CatalyzerRuntime::builder()
            .setup_tokio(|b|
                b.enable_all()
                .thread_name_fn(|| {
                    let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                    format!("Catalyzer Runtime Worker #{id}")
                })
            )?
            .build()
    }
    /// Creates a new builder for the runtime.
    #[inline]
    pub fn builder() -> CatalyzerRuntimeBuilder {
        CatalyzerRuntimeBuilder {
            tokio: None,
        }
    }
    /// Initializes the runtime with an optional custom initialization function.
    pub fn init(func: Option<fn() -> Result<Self>>) -> Self {
        match func.map_or_else(Self::default_preinit, |f| f()) {
            Err(e) => {
                log::error!("Failed to initialize runtime: {}", e);
                std::process::exit(1);
            }
            Ok(rt) => rt,
        }
    }
    /// Runs the given future on the runtime.
    /// 
    /// This function will also install signal handlers for Ctrl+C and SIGTERM.
    ///
    /// # Example
    /// 
    /// ```rust
    /// # use catalyzer::internals::runtime::CatalyzerRuntime;
    /// # use catalyzer::Result;
    /// fn main() {
    ///     async fn main() -> Result {
    ///         // Your code here
    ///         Ok(())
    ///     }
    ///     CatalyzerRuntime::init(None).run(main);
    /// }
    /// ```
    pub fn run<F, Fut>(self, f: F) where
        Fut: Future<Output = Result>,
        F: FnOnce() -> Fut,
    {
        let (sender, reciever) = tokio::sync::oneshot::channel::<()>();
        let mercy_handlers = async {
            tokio::select! {
                _ = signals::ctrl_c() => {
                    log::info!("Received Ctrl+C, shutting down...");
                },
                _ = signals::term() => {
                    log::info!("Received SIGTERM, shutting down...");
                },
            }
            tokio::select! {
                _ = signals::ctrl_c() => {},
                _ = signals::term() => {},
            }
            log::warn!("Received second signal, please mercy...");
            if let Err(_) = sender.send(()) {
                log::error!("Failed to emit mercy signal, shutting down...");
                std::process::exit(1);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            log::error!("Mercy timeout reached, shutting down...");
            std::process::exit(1);
        };
        self.tokio.spawn(mercy_handlers);
        self.tokio.block_on(async move {
            tokio::select! {
                _ = f() => {
                    log::debug!("Webserver shutdown successfully!");
                },
                _ = reciever => {
                    log::trace!("Received mercy signal, shutting down...");
                },
            }
        });
        self.tokio.shutdown_timeout(tokio::time::Duration::from_secs(5));
        log::info!("Shutdown successful!");
    }
}

impl CatalyzerRuntimeBuilder {
    /// Allows you to set up the Tokio runtime.
    /// 
    /// This function is chainable.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use catalyzer::internals::runtime::CatalyzerRuntimeBuilder;
    /// # use catalyzer::Result;
    /// # fn main() -> Result {
    /// CatalyzerRuntime::builder()
    ///     .setup_tokio(|b| b.enable_all())?
    ///     .build()
    /// # ;
    /// # }
    pub fn setup_tokio<F>(mut self, f: F) -> Result<Self> where
        F: FnOnce(&mut TokioRuntimeBuilder) -> &mut TokioRuntimeBuilder,
    {
        let mut builder = TokioRuntimeBuilder::new_multi_thread();
        f(&mut builder);
        builder.build()
            .map(|t| { self.tokio = Some(t); self})
            .map_auto()
    }
    /// Builds the [`CatalyzerRuntime`](crate::internals::runtime::CatalyzerRuntime).
    /// 
    /// This function consumes the builder, and returns a runtime.
    pub fn build(self) -> Result<CatalyzerRuntime> {
        let tokio = self.tokio.ok_or(CatalyzerError::RuntimeInitializationError)?;
        Ok(CatalyzerRuntime { tokio, })
    }
}

pub(crate) mod signals {
    use tokio::signal;
    pub(crate) async fn ctrl_c() {
        if let Err(_) = signal::ctrl_c().await {
            log::error!("Failed to install signal handler");
            std::process::exit(1);
        }
    }
    #[cfg(unix)]
    pub(crate) async fn term() {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => { stream.recv().await; },
            Err(e) => {
                log::error!("Failed to install signal handler: {}", e);
                std::process::exit(1);
            },
        }
    }
    #[cfg(not(unix))]
    pub(crate) async fn term() {
        core::future::pending::<()>().await;
    }
}
