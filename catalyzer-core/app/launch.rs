use std::net::ToSocketAddrs as ToAddrs;
use core::convert::Infallible;

use axum::serve::WithGracefulShutdown;

use crate::__internals__::*;
use crate::http::*;
use crate::res::*;
use crate::req::*;
use crate::*;

#[cfg(feature = "automatic-defaults")]
mod default;

/// The type of a launched application.
#[repr(transparent)]
pub struct CatalyzedApp<S, State = ()>(
    WithGracefulShutdown<AxumRouter<State>, S, shutdown_signal>
) where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send;

impl<S, State> core::fmt::Debug for CatalyzedApp<S, State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CatalyzedApp")
            .field(&"...")
            .finish()
    }
}

/// The type of a bound application.
/// 
/// This only exists to allow for the `catalyze` method to be called.
/// 
/// Essentially, this is the internal Router type,
/// with the address it's bound to (what it's listening on) attached.
pub struct BoundApp<State = ()> {
    router: AxumRouter<State>,
    addr: std::net::SocketAddr,
}

impl<S, State> core::fmt::Debug for BoundApp<State> where 
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BoundApp")
            .field("address", &self.addr)
            .finish()
    }
}

impl<S, State> core::future::IntoFuture for CatalyzedApp<S, State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    type Output = <WithGracefulShutdown<AxumRouter<State>, S, shutdown_signal> as IntoFuture>::Output;
    type IntoFuture = <WithGracefulShutdown<AxumRouter<State>, S, shutdown_signal> as IntoFuture>::IntoFuture;
    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        self.0.into_future()
    }
}

impl<State> App<State> where
    State: Default + Clone + Send + Sync + 'static,
{
    /// Launches the application at the given address.
    pub fn launch<S2, Addr>(self, addr: Addr) -> Result<BoundApp<S2>> where
        Addr: ToAddrs,
    {
        let addr = addr.to_socket_addrs()?
            .next()
            .ok_or(CatalyzerError::NO_ADDRESS)?;
        log::debug!("launching server at {}", addr);
        #[cfg(not(feature = "automatic-defaults"))]
        {
            return Ok(BoundApp {
                router: self.router.with_state(State::default()),
                addr,
            })
        }
        #[cfg(feature = "automatic-defaults")]
        {
            if !self.modified {
                return Ok(BoundApp {
                    router: default::make_default_router().with_state(State::default()),
                    addr,
                })
            }
            Ok(BoundApp {
                router: self.router.with_state(State::default()),
                addr,
            })
        }
    }
}

impl<S, State> BoundApp<State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S>,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    /// Catalyzes the application, and starts the server.
    ///
    /// **DO NOT CALL THIS FUNCTION DIRECTLY!**
    /// 
    /// This function will automatically be called by the [`main`] macro.
    /// 
    /// [`main`]: attr.main.html
    pub async fn catalyze(self) -> Result<CatalyzedApp<S, State>> {
        let tcp = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(CatalyzerError::from)?;
        let app = axum::serve(tcp, self.router);
        Ok(CatalyzedApp(app.with_graceful_shutdown(shutdown_signal())))
    }
}

#[allow(non_camel_case_types)]
type shutdown_signal = core::pin::Pin<Box<dyn
    core::future::Future<Output = ()> + Send
>>;
fn shutdown_signal() -> shutdown_signal {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        log::info!("Received Ctrl+C, shutting down...");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        log::info!("Received SIGTERM, shutting down...");
    };

    #[cfg(not(unix))]
    let terminate = core::future::pending::<()>();

    Box::pin(async {
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        log::debug!("Shut down complete!");
        std::process::exit(0);
    })
}
