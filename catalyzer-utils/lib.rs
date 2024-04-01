//! Utilities to make development easier.

#![no_std]

/// An extension trait for [`Result`]s.
/// 
/// [`Result`]: core::result::Result
pub trait ResultExt<O, E> {
    /// Convert the result into another result type.
    fn map_auto<O2, E2>(self) -> Result<O2, E2> where
        O2: From<O>,
        E2: From<E>;
}

/// An extension trait for [`Option`]s.
/// 
/// [`Option`]: core::option::Option
pub trait OptionExt<T> {
    /// Unwraps the option, or returns a default value (converted to the option's type).
    fn unwrap_or_auto<T2>(self, default: T2) -> T2 where
        T2: From<T>;
    /// Maps the option to another option type.
    fn map_auto<T2>(self) -> Option<T2> where
        T2: From<T>;
    /// Unwraps the option, or returns an error.
    fn ok_or_auto<O, E>(self, err: E) -> Result<O, E> where
        O: From<T>;
}

impl<O, E> ResultExt<O, E> for Result<O, E> {
    #[inline]
    fn map_auto<O2, E2>(self) -> Result<O2, E2> where
        O2: From<O>,
        E2: From<E>
    {
        match self {
            Err(e) => Err(E2::from(e)),
            Ok(o) => Ok(O2::from(o)),
        }
    }
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn unwrap_or_auto<T2>(self, default: T2) -> T2 where
        T2: From<T>
    {
        match self {
            None => default,
            Some(t) => T2::from(t),
        }
    }

    #[inline]
    fn map_auto<T2>(self) -> Option<T2> where
        T2: From<T>
    {
        match self {
            None => None,
            Some(t) => Some(T2::from(t)),
        }
    }

    #[inline]
    fn ok_or_auto<O, E>(self, err: E) -> Result<O, E> where
        O: From<T>
    {
        match self {
            None => Err(err),
            Some(t) => Ok(O::from(t)),
        }
    }
}
