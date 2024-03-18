//! Content loaders for strings and bytes.

use utils::ResultTransformer;
use axum::body::Body;
use std::path::Path;
use crate::Result;
use core::fmt;

macro_rules! loader {
    ($(
        $(#[$attr:meta])*
        $name:ident -> $inner:ident(
            $(#[$alloc_attr:meta])*
            ;$(#[$alloc_fn_attr:meta])*
            $alloc:ty;
            $(#[$static_attr:meta])*
            ;$(#[$static_fn_attr:meta])*
            $static:ty;
        )
    )+) => ($(
        $(#[$attr])*
        #[repr(transparent)]
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($inner::$name);
        impl fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl From<$alloc> for $name {
            #[inline]
            fn from(s: $alloc) -> Self {
                Self($inner::$name::Alloc(s))
            }
        }
        impl From<$static> for $name {
            #[inline]
            fn from(s: $static) -> Self {
                Self($inner::$name::Static(s))
            }
        }
        impl From<$name> for Body {
            #[inline]
            fn from(this: $name) -> Self {
                Body::from(this.0)
            }
        }
        impl $name {
            $(#[$alloc_fn_attr])*
            #[inline]
            pub const fn new_static(s: $static) -> Self {
                Self($inner::$name::Static(s))
            }
            $(#[$static_fn_attr])*
            #[inline]
            pub fn new_alloc(s: $alloc) -> Self {
                Self($inner::$name::Alloc(s))
            }
        }
        mod $inner {
            use super::*;
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) enum $name {
                $(#[$alloc_attr])*
                Alloc($alloc),
                $(#[$static_attr])*
                Static($static),
            }
            impl From<$name> for Body {
                #[inline]
                fn from(this: $name) -> Self {
                    match this {
                        $name::Alloc(s) => Body::from(s),
                        $name::Static(s) => Body::from(s),
                    }
                }
            }
        }
    )+);
}

loader!(
    /// A content loader for strings.
    StringContentLoader -> string_inner(
        /// An allocated String.
        ;/// Creates a new `StringContentLoader` from an allocated `String`.
        String;
        /// A static string.
        ;/// Creates a new `StringContentLoader` from a static string.
        &'static str;
    )
    /// A content loader for bytes.
    BytesContentLoader -> bytes_inner(
        /// An allocated Vec<u8>.
        ;/// Creates a new `BytesContentLoader` from an allocated `Vec<u8>`.
        Vec<u8>;
        /// A static byte slice.
        ;/// Creates a new `BytesContentLoader` from a static byte slice.
        &'static [u8];
    )
);

impl StringContentLoader {
    /// Creates a new `StringContentLoader` from a file.
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        tokio::fs::read_to_string(path).await.map_auto()
    }
}

impl BytesContentLoader {
    /// Creates a new `BytesContentLoader` from a file.
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        tokio::fs::read(path).await.map_auto()
    }
}

impl fmt::Display for StringContentLoader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            string_inner::StringContentLoader::Alloc(s) => s.fmt(f),
            string_inner::StringContentLoader::Static(s) => s.fmt(f),
        }
    }
}

impl fmt::Display for BytesContentLoader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            bytes_inner::BytesContentLoader::Alloc(s) => String::from_utf8_lossy(s).fmt(f),
            bytes_inner::BytesContentLoader::Static(s) => String::from_utf8_lossy(s).fmt(f),
        }
    }
}
