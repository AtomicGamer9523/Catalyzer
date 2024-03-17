use std::net::{SocketAddr, ToSocketAddrs};
use axum::Router as AxumRouter;
use crate::__internals__::*;
use crate::error::*;

pub(crate) mod launch;

#[derive(Debug)]
pub struct App<State = ()> {
    router: AxumRouter<State>,
    address: Option<SocketAddr>,
    https_address: Option<SocketAddr>,
}

impl<State> App<State> where
    State: Clone + Send + Sync + 'static
{
    pub fn new() -> Self {
        Self {
            router: AxumRouter::<State>::new(),
            address: None,
            https_address: None,
        }
    }
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
    pub fn set_state<S2>(self, state: State) -> App<S2> {
        App {
            router: self.router.with_state::<S2>(state),
            address: self.address,
            https_address: self.https_address,
        }
    }
}
