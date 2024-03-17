//! Utilities to make development easier.

#![no_std]

/// A trait to convert one result type into another.
pub trait ResultTransformer<O2, E2> {
    /// Convert the result into another result type.
    fn map_auto(self) -> Result<O2, E2>;
}

impl<O, E, O2, E2> ResultTransformer<O2, E2> for Result<O, E> where
    O2: From<O>,
    E2: From<E>,
{
    #[inline]
    fn map_auto(self) -> Result<O2, E2> {
        self.map(From::from).map_err(From::from)
    }
}
