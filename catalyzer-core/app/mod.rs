use std::net::{SocketAddr, ToSocketAddrs};
use axum::Router as AxumRouter;
use crate::__internals__::*;
use crate::error::*;

pub(crate) mod launch;

/// The main application type.
/// 
/// See the [module-level documentation](crate::app) for more information.
#[derive(Debug)]
pub struct App<State = ()> {
    router: AxumRouter<State>,
    address: Option<SocketAddr>,
    https_address: Option<SocketAddr>,
}

impl<State> App<State> where
    State: Clone + Send + Sync + 'static
{
    /// Creates a new [`App`] instance.
    /// 
    /// This is the main entry point for creating a new application.
    /// 
    /// It is recommended to use the [`App!`] macro instead of this method.
    /// 
    /// [`App`]: crate::App
    /// [`App!`]: macro.App.html
    pub fn new() -> Self {
        Self {
            router: AxumRouter::<State>::new(),
            address: None,
            https_address: None,
        }
    }
    /// Mounts a route handler on the application.
    /// 
    /// This requires a handler that implements the [`AxumHandler`] trait.
    /// Additionally, you need to provide a metadata type that implements the
    /// [`HandlerMetadata`] trait.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use catalyzer::*;
    /// # #[main]
    /// # fn main() -> Result {
    /// #[get("/")]
    /// fn index() {
    ///     "Hello, world!"
    /// }
    /// 
    /// let app = App::new()
    ///     .route::<_, index_metadata, _>(index)?;
    /// # }
    /// 
    pub fn route<Return, Meta, Handler>(
        mut self,
        handler: Handler
    ) -> Result<Self> where
        Handler: AxumHandler<Return, State>,
        Meta: HandlerMetadata,
        Return: 'static
    {
        let method_router = match Meta::METHOD {
            Method::GET => axum::routing::get(handler),
            Method::POST => axum::routing::post(handler),
            Method::PUT => axum::routing::put(handler),
            Method::DELETE => axum::routing::delete(handler),
            Method::PATCH => axum::routing::patch(handler),
            Method::HEAD => axum::routing::head(handler),
            Method::OPTIONS => axum::routing::options(handler),
            Method::TRACE => axum::routing::trace(handler),
            _ => return Err(CatalyzerError::UnsupportedMethodError)
        };
        log::trace!("Mounted a {} on \"{}\"", Meta::METHOD, Meta::PATH);
        self.router = self.router.route(Meta::PATH, method_router);
        Ok(self)
    }
    /// Binds the application to a specific address.
    /// 
    /// This is required before launching the application.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use catalyzer::*;
    /// # #[main]
    /// # fn main() -> Result {
    /// let app = App::new().bind("0.0.0.0:8080")?;// Localhost on port 8080
    /// # }
    pub fn bind<Addr>(mut self, addr: Addr) -> Result<Self> where
        Addr: ToSocketAddrs
    {
        let mut addrs = addr.to_socket_addrs()?;
        let addr = addrs.next().ok_or(IoError::new(
            IoErrorKind::AddrNotAvailable,
            "No addresses found for the provided address"
        ))?;
        
        log::debug!("Binding to {}", addr);
        self.address = Some(addr);
        Ok(self)
    }
    /// Sets the state of the application.
    /// 
    /// If your application requires a state, you must set it using this method.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use catalyzer::*;
    /// struct AppState {
    ///     counter: u32,
    /// }
    /// 
    /// # #[main]
    /// # fn main() -> Result {
    /// let app = App::new()
    ///     .set_state(AppState { counter: 0 });
    /// # }
    pub fn set_state<S2>(self, state: State) -> App<S2> {
        App {
            router: self.router.with_state::<S2>(state),
            address: self.address,
            https_address: self.https_address,
        }
    }
}
