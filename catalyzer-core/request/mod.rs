use axum::body::Body;

pub type RawRequest<T = Body> = ::axum::extract::Request<T>;
