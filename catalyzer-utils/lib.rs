//! Utilities to make development easier.

#![no_std]

pub trait ResultTransformer<O2, E2> {
    fn auto(self) -> Result<O2, E2>;
}

impl<O, E, O2, E2> ResultTransformer<O2, E2> for Result<O, E> where
    O: Into<O2>,
    E: Into<E2>
{
    #[inline]
    fn auto(self) -> Result<O2, E2> {
        self.map(Into::into).map_err(Into::into)
    }
}
