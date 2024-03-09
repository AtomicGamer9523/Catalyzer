use std::net::ToSocketAddrs as ToAddrs;
use core::convert::Infallible;

use axum::serve::WithGracefulShutdown;
use axum::Router as AxumRouter;

use crate::http::*;
use crate::res::*;
use crate::req::*;
use crate::*;

/// The type of a launched application.
#[repr(transparent)]
#[allow(missing_debug_implementations)]
pub struct Launch<S, State = ()>(
    WithGracefulShutdown<AxumRouter<State>, S, shutdown_signal>
) where
    State: Clone + Send + Sync + 'static,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send;

impl<S, State> core::future::IntoFuture for Launch<S, State> where
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

impl<S, State> App<State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S>,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    /// Launches the application at the given address.
    pub async fn launch<Addr: ToAddrs>(self, addr: Addr) -> Result<Launch<S, State>> {
        use axum::serve::*;
        let addr = addr.to_socket_addrs()?
            .next()
            .ok_or(CatalyzerError::NO_ADDRESS)?;
        let tcp = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(CatalyzerError::from)?;
        Ok(Launch(
            serve(tcp, self.0)
            .with_graceful_shutdown(shutdown_signal())
        ))
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
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = core::future::pending::<()>();

    Box::pin(async {
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    })
}
