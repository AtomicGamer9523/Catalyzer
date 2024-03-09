use ::axum::handler::Handler;
pub(crate) mod launch;

/// The main application type.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct App<State = ()>(pub(super) axum::Router<State>);

macro_rules! method {
    ( $(
        $(#[$attr:meta])*
        $name:ident
    )* ) => ( $(
        $(#[$attr])*
        pub fn $name<Return, H>(mut self, path: &str, handler: H) -> Self
        where
            H: Handler<Return, State>,
            Return: Clone + Send + Sync + 'static,
        {
            self.0 = self.0.route(path, axum::routing::$name(handler));
            self
        }
    )* )
}

impl<State> App<State> where
    State: Clone + Send + Sync + 'static
{
    /// Construct a new `App`.
    #[inline]
    pub fn new() -> Self {
        Self(axum::Router::new())
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
}
