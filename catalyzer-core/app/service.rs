use crate::http::*;
use super::*;

impl<State> App<State> where
    State: Clone + Send + Sync + 'static
{
    /// Adds a [`CatalyzerService`] to the application.
    /// 
    /// [`CatalyzerService`]: crate::http::CatalyzerService
    #[inline]
    pub fn service<S>(mut self, service: S) -> Self
    where
        S: CatalyzerService<RawRequest, Error = Infallible> + Clone + Send + Sync + 'static,
        S::Response: IntoRawResponse,
        S::Future: Send + 'static,
    {
        self.router = service.mount_self(self.router);
        self
    }
}

/// A service that Catalyzer can use.
pub trait CatalyzerService<Req>: Service<Req> {
    /// Mounts the service onto the given router.
    fn mount_self<S>(self, router: AxumRouter<S>) -> AxumRouter<S> where
        S: Clone + Send + Sync + 'static;
}
