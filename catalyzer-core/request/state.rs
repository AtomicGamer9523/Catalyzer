use ::axum::extract::{FromRef, FromRequestParts};
use crate::http::*;
use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

/// Request that uses state
#[derive(Debug, Default, Clone, Copy)]
pub struct State<S>(pub S);

impl<OuterState, InnerState> FromRequestParts<OuterState> for State<InnerState>
where
    InnerState: FromRef<OuterState>,
    OuterState: Send + Sync,
{
    type Rejection = Infallible;

    fn from_request_parts<
        'a, 'b, 'c
    >(
        _parts: &'a mut request::Parts,
        state: &'b OuterState
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'c>>
    where
        'a: 'c,
        'b: 'c,
        Self: 'c,
    {
        Box::pin(async move {
            let inner_state = InnerState::from_ref(state);
            Ok(Self(inner_state))
        })
    }
}

impl<S> Deref for State<S> {
    type Target = S;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for State<S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
