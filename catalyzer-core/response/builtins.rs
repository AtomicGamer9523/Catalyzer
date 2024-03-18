use super::*;

macro_rules! content_types {
    ($(
        $(#[$attr:meta])*
        type $name:ident($content_loader:ident -> $content_type:literal) {
            $(#[$alloc_attr:meta])*
            fn new_static($static:ty);
            $(#[$static_fn_attr:meta])*
            fn new_alloc($alloc:ty);
            $(#[$from_file_attr:meta])*
            fn from_file();
        }
    )+) => ($(
        $(#[$attr])*
        #[repr(transparent)]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name($content_loader);

        impl $name {
            $(#[$alloc_attr])*
            #[inline]
            pub const fn new_static(s: $static) -> Self {
                Self($content_loader::new_static(s))
            }
            $(#[$static_fn_attr])*
            #[inline]
            pub fn new_alloc(s: $alloc) -> Self {
                Self($content_loader::new_alloc(s))
            }
            $(#[$from_file_attr])*
            #[inline]
            pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
                $content_loader::from_file(path).await.map(Self)
            }
        }

        impl From<$alloc> for $name {
            #[inline]
            fn from(this: $alloc) -> Self {
                Self::new_alloc(this)
            }
        }

        impl From<$static> for $name {
            #[inline]
            fn from(this: $static) -> Self {
                Self::new_static(this)
            }
        }

        impl fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<$name> for Body {
            #[inline]
            fn from(this: $name) -> Self {
                Body::from(this.0)
            }
        }

        impl ::axum::response::IntoResponse for $name {
            fn into_response(self) -> RawResponse {
                RawResponse::builder()
                    .header("Content-Type", $content_type)
                    .body(self.into())
                    .unwrap_or_default()
            }
        }
    )+);
}

content_types! {
    /// A type representing HTML content.
    type Html(StringContentLoader -> "text/html") {
        /// Create a new `Html` from a static string.
        fn new_static(&'static str);
        /// Create a new `Html` from an allocated string.
        fn new_alloc(String);
        /// Create a new `Html` from a file.
        fn from_file();
    }
    /// A type representing CSS content.
    type Css(StringContentLoader -> "text/css") {
        /// Create a new `Css` from a static string.
        fn new_static(&'static str);
        /// Create a new `Css` from an allocated string.
        fn new_alloc(String);
        /// Create a new `Css` from a file.
        fn from_file();
    }
    /// A type representing JavaScript content.
    type Js(StringContentLoader -> "application/javascript") {
        /// Create a new `Js` from a static string.
        fn new_static(&'static str);
        /// Create a new `Js` from an allocated string.
        fn new_alloc(String);
        /// Create a new `Js` from a file.
        fn from_file();
    }
    /// A type representing JSON content.
    type Json(StringContentLoader -> "application/json") {
        /// Create a new `Json` from a static string.
        fn new_static(&'static str);
        /// Create a new `Json` from an allocated string.
        fn new_alloc(String);
        /// Create a new `Json` from a file.
        fn from_file();
    }
    /// A type representing XML content.
    type Xml(StringContentLoader -> "application/xml") {
        /// Create a new `Xml` from a static string.
        fn new_static(&'static str);
        /// Create a new `Xml` from an allocated string.
        fn new_alloc(String);
        /// Create a new `Xml` from a file.
        fn from_file();
    }
    /// A type representing plain text content.
    type Text(StringContentLoader -> "text/plain") {
        /// Create a new `Text` from a static string.
        fn new_static(&'static str);
        /// Create a new `Text` from an allocated string.
        fn new_alloc(String);
        /// Create a new `Text` from a file.
        fn from_file();
    }
    /// A type representing binary content.
    type Binary(BytesContentLoader -> "application/octet-stream") {
        /// Create a new `Binary` from a static byte slice.
        fn new_static(&'static [u8]);
        /// Create a new `Binary` from an allocated `Vec<u8>`.
        fn new_alloc(Vec<u8>);
        /// Create a new `Binary` from a file.
        fn from_file();
    }
}
