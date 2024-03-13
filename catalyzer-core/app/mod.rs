pub(crate) mod launch;
pub(crate) mod service;

use crate::__internals__::*;
use crate::res::*;
use crate::req::*;

use core::convert::Infallible;

/// The main application type.
#[derive(Debug, Clone)]
#[cfg_attr(not(feature = "automatic-defaults"), repr(transparent))]
pub struct App<State = ()> {
    pub(super) router: AxumRouter<State>,
    #[cfg(feature = "automatic-defaults")]
    pub(super) modified: bool,
}

macro_rules! method {
    ( $(
        $(#[$attr:meta])*
        $name:ident
    )* ) => ( $(
        $(#[$attr])*
        pub fn $name<Return, H>(mut self, path: &str, handler: H) -> Self
        where
            H: AxumHandler<Return, State>,
            Return: Clone + Send + Sync + 'static,
        {
            self.router = self.router.route(path, axum::routing::$name(handler));
            #[cfg(feature = "automatic-defaults")]
            { self.modified = true; }
            self
        }
    )* )
}

impl App<()> {
    /// Construct a new stateless `App`.
    #[inline]
    pub fn new_stateless() -> Self {
        Self {
            router: AxumRouter::new(),
            #[cfg(feature = "automatic-defaults")]
            modified: false,
        }
    }
}

#[cfg(feature = "automatic-defaults")]
impl Default for App<()> {
    #[inline]
    fn default() -> Self {
        Self::new_stateless()
    }
}

impl<State> App<State> where
    State: Clone + Send + Sync + 'static
{
    /// Construct a new `App`.
    #[inline]
    pub fn new() -> Self {
        Self {
            router: AxumRouter::new(),
            #[cfg(feature = "automatic-defaults")]
            modified: false,
        }
    }

    method!(
        /// Add a `GET` route to the application.
        get
        /// Add a `POST` route to the application.
        post
        /// Add a `PUT` route to the application.
        put
        /// Add a `DELETE` route to the application.
        delete
        /// Add a `PATCH` route to the application.
        patch
        /// Add a `HEAD` route to the application.
        head
        /// Add a `OPTIONS` route to the application.
        options
    );

    /// Adds a fallback handler to the application.
    /// 
    /// The fallback handler is called when no route matches the incoming request.
    /// Or if any of the handlers return `Err`.
    #[inline]
    pub fn fallback<Return, H>(mut self, handler: H) -> Self
    where
        H: AxumHandler<Return, State>,
        Return: Clone + Send + Sync + 'static,
    {
        self.router = self.router.fallback(handler);
        self
    }
    /// Sets the application's state.
    #[inline]
    pub fn set_state<S2>(self, state: State) -> App<S2> {
        App {
            router: self.router.with_state::<S2>(state),
            #[cfg(feature = "automatic-defaults")]
            modified: true,
        }
    }
}
