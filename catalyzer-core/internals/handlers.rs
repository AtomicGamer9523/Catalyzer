pub use ::axum::handler::Handler as AxumHandler;
pub use ::axum::http::Method;

pub trait HandlerMetadata {
    const PATH: &'static str;
    const METHOD: Method;
}
