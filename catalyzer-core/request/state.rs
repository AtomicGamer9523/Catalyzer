use axum::extract::{FromRequestParts, FromRef};
use axum::http::request::Parts as RequestParts;
use core::convert::Infallible;
use std::future::Future;

/// An extractor that extracts the state of the application.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct State<S>(S);

impl<OuterState, InnerState> FromRequestParts<OuterState> for State<InnerState>
where
    InnerState: FromRef<OuterState>,
    OuterState: Send + Sync,
{
    type Rejection = Infallible;
    fn from_request_parts<'a: 'c, 'b: 'c, 'c>(
        _: &'a mut RequestParts,
        state: &'b OuterState,
    ) -> core::pin::Pin<Box<
        dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'c,
    >> where Self: 'c {
        Box::pin(async move {
            Ok(Self(InnerState::from_ref(state)))
        })
    }
}

impl<S> core::ops::Deref for State<S> {
    type Target = S;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> core::ops::DerefMut for State<S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
