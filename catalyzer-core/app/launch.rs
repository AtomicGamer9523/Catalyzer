//! 

use core::convert::Infallible;
use core::future::*;
use core::pin::Pin;

use axum::serve::*;
use tower::Service;

use crate::res::*;
use crate::req::*;
use super::*;

type F1 = Pin<Box<dyn Future<Output = ()> + Send>>;
type Ca<S, State> = WithGracefulShutdown<AxumRouter<State>, S, F1>;

/// A Catalyzed application that is ready to be launched.
#[repr(transparent)]
#[allow(missing_debug_implementations)]
pub struct CatalyzedApp<S, State = ()>(Ca<S, State>) where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send;
impl<S, State> IntoFuture for CatalyzedApp<S, State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send, {
    type Output = <Ca<S, State> as IntoFuture>::Output;
    type IntoFuture = <Ca<S, State> as IntoFuture>::IntoFuture;
    #[inline] fn into_future(self) -> Self::IntoFuture { self.0.into_future() } }
impl<S, State> App<State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    /// Catalyzes the application and launches it.
    /// 
    /// This should be the last method called on the [`App`](crate::App) instance.
    pub async fn launch(self) -> Result<CatalyzedApp<S, State>> {
        let addr = self.address.ok_or(CatalyzerError::NoAddress)?;
        let tcp = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(CatalyzerError::from)?;
        let app = axum::serve(tcp, self.router);
        Ok(CatalyzedApp(app.with_graceful_shutdown(signal_handler())))
    }
}

#[inline]
fn signal_handler() -> Pin<Box<dyn Future<Output = ()> + Send>> {
    use super::runtime::signals::*;
    Box::pin(async { tokio::select! {
        _ = ctrl_c() => {},
        _ = term() => {},
    } })
}
