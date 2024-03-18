use core::convert::Infallible;
use core::future::*;
use core::pin::Pin;

use axum::serve::*;
use tower::Service;

use crate::res::*;
use crate::req::*;
use super::*;

/// The type of a launched application.
/// 
/// Docs will be made by [@Phabr1945](https://github.com/Phabr1945)
#[repr(transparent)]
#[allow(missing_debug_implementations)]
pub struct CatalyzedApp<S, State = ()>(WithGracefulShutdown<AxumRouter<State>, S, Pin<Box<dyn Future<Output = ()> + Send>>>) where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send;
impl<S, State> core::future::IntoFuture for CatalyzedApp<S, State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send, {
    type Output = <WithGracefulShutdown<AxumRouter<State>, S, Pin<Box<dyn Future<Output = ()> + Send>>> as IntoFuture>::Output;
    type IntoFuture = <WithGracefulShutdown<AxumRouter<State>, S, Pin<Box<dyn Future<Output = ()> + Send>>> as IntoFuture>::IntoFuture;
    #[inline] fn into_future(self) -> Self::IntoFuture { self.0.into_future() }
}
impl<S, State> App<State> where
    State: Clone + Send + Sync + 'static,
    AxumRouter<State>: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S> + Send + 'static,
    for<'a> <AxumRouter<State> as Service<IncomingStream<'a>>>::Future: Send,
    S: Service<RawRequest, Response = RawResponse, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
{
    /// Catalyzes the application and launches it.
    /// 
    /// This should be the last method called on the `App` instance.
    /// 
    /// Docs will be made by [@Phabr1945](https://github.com/Phabr1945)
    pub async fn launch(self) -> Result<CatalyzedApp<S, State>> {
        let addr = self.address.ok_or(CatalyzerError::NoAddress)?;
        let tcp = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(CatalyzerError::from)?;
        let app = axum::serve(tcp, self.router);
        Ok(CatalyzedApp(app.with_graceful_shutdown(crate::runtime::signal_handler())))
    }
}
