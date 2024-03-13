use ::axum::response::IntoResponse;

#[cfg(feature = "file-loading-dynamic")]
use ::std::path::Path;
#[cfg(feature = "file-loading-dynamic")]
use crate::Result;

pub use ::axum::response::IntoResponse as IntoRawResponse;
pub use ::axum::response::Response as RawResponse;

#[cfg(all(
    feature = "file-loading-dynamic",
    feature = "file-loading-embed"
))]
compile_error!("The `file-loading-dynamic` and `file-loading-embed` features are mutually exclusive.");
#[cfg(all(
    not(feature = "file-loading-dynamic"),
    not(feature = "file-loading-embed")
))]
compile_error!("At least one of the `file-loading-dynamic` and `file-loading-embed` features must be enabled.");

macro_rules! custom_content_type {
    () => ();
    (@Common $(#[$attr:meta])* $dyn:ty;$sta:ty;$name:ident;$content_type:literal) => (
        $(#[$attr])*
        #[cfg(feature = "file-loading-dynamic")]
        impl From<$dyn> for $name {
            #[inline]
            fn from(this: $dyn) -> $name {
                Self(this)
            }
        }
        $(#[$attr])*
        #[cfg(feature = "file-loading-dynamic")]
        impl From<$name> for $dyn {
            #[inline]
            fn from(this: $name) -> $dyn {
                this.0
            }
        }
        $(#[$attr])*
        #[cfg(feature = "file-loading-embed")]
        impl From<$sta> for $name {
            #[inline]
            fn from(this: $sta) -> $name {
                Self(this)
            }
        }
        $(#[$attr])*
        #[cfg(feature = "file-loading-embed")]
        impl From<$name> for $sta {
            #[inline]
            fn from(this: $name) -> $sta {
                this.0
            }
        }
        $(#[$attr])*
        impl $name {
            /// Allows to create a new instance from the given value at compile time.
            /// 
            /// This method is only available when the `file-loading-embed` feature is enabled.
            #[inline]
            #[cfg(feature = "file-loading-embed")]
            pub const fn new(s: $sta) -> Self {
                Self(s)
            }
        }
        $(#[$attr])*
        impl IntoResponse for $name {
            fn into_response(self) -> ::axum::response::Response<::axum::body::Body> {
                ::axum::response::Response::builder()
                    .header(::axum::http::header::CONTENT_TYPE, $content_type)
                    .body(self.0.into())
                    .unwrap_or_default()
            }
        }
    );
    (
        $(#[$attr:meta])*
        type $name:ident($content_type:literal; String)
        $($rest:tt)*
    ) => (
        $(#[$attr])*
        #[repr(transparent)]
        #[cfg(feature = "file-loading-dynamic")]
        #[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);
        $(#[$attr])*
        #[cfg(feature = "file-loading-dynamic")]
        impl From<&str> for $name {
            #[inline]
            fn from(this: &str) -> $name {
                Self(this.into())
            }
        }
        $(#[$attr])*
        #[repr(transparent)]
        #[cfg(feature = "file-loading-embed")]
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(&'static str);
        custom_content_type!(@Common $(#[$attr])* String; &'static str; $name; $content_type);
        $(#[$attr])*
        impl $name {
            /// Allows to dynamically load a file at runtime,
            /// Then submits it as a response with the given content type.
            /// 
            /// This method is only available when the `file-loading-dynamic` feature is enabled.
            #[cfg(feature = "file-loading-dynamic")]
            pub async fn from_file<P>(path: P) -> Result<$name> where
                P: AsRef<Path>,
            {
                utils::ResultTransformer::auto(
                    tokio::fs::read_to_string(path).await
                )
            }
        }
        custom_content_type!($($rest)*);
    );
    (
        $(#[$attr:meta])*
        type $name:ident($content_type:literal; Bytes)
        $($rest:tt)*
    ) => (
        $(#[$attr])*
        #[repr(transparent)]
        #[cfg(feature = "file-loading-dynamic")]
        #[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(Vec<u8>);
        $(#[$attr])*
        #[repr(transparent)]
        #[cfg(feature = "file-loading-embed")]
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(&'static [u8]);
        custom_content_type!(@Common $(#[$attr])* Vec<u8>; &'static [u8]; $name; $content_type);
        $(#[$attr])*
        impl $name {
            /// Allows to dynamically load a file at runtime,
            /// Then submits it as a response with the given content type.
            /// 
            /// This method is only available when the `file-loading-dynamic` feature is enabled.
            #[cfg(feature = "file-loading-dynamic")]
            pub async fn from_file<P>(path: P) -> Result<$name> where
                P: AsRef<Path>,
            {
                use tokio::io::AsyncReadExt;
                let mut contents = vec![];
                let mut file = tokio::fs::OpenOptions::new()
                    .write(false)
                    .create(false)
                    .read(true)
                    .open(path)
                    .await?;
                file.read_to_end(&mut contents).await?;
                Ok(Self(contents.into()))
            }
        }
        custom_content_type!($($rest)*);
    );
}

custom_content_type! {
    /// Represents the `text/html` content type.
    type Html("text/html"; String)
    /// Represents the `text/css` content type.
    type Css("text/css"; String)
    /// Represents the `application/javascript` content type.
    type Js("application/javascript"; String)
    /// Represents the `application/json` content type.
    type Json("application/json"; String)
    /// Represents the `application/xml` content type.
    #[cfg(feature = "more-content-types")]
    type Xml("application/xml"; String)
    /// Represents the `image/svg+xml` content type.
    type Svg("image/svg+xml"; String)
    /// Represents the `image/png` content type.
    type Png("image/png"; Bytes)
    /// Represents the `image/jpeg` content type.
    type Jpeg("image/jpeg"; Bytes)
    /// Represents the `image/gif` content type.
    type Gif("image/gif"; Bytes)
    /// Represents the `image/webp` content type.
    type Webp("image/webp"; Bytes)
    /// Represents the `image/x-icon` content type.
    #[cfg(feature = "more-content-types")]
    type Bmp("image/bmp"; Bytes)
    /// Represents the `image/x-icon` content type.
    type Ico("image/x-icon"; Bytes)
    /// Represents the `image/x-icon` content type.
    #[cfg(feature = "more-content-types")]
    type Tiff("image/tiff"; Bytes)
    /// Represents the `image/x-icon` content type.
    type Wav("audio/wav"; Bytes)
    /// Represents the `audio/mpeg` content type.
    type Mp3("audio/mpeg"; Bytes)
    /// Represents the `audio/ogg` content type.
    #[cfg(feature = "more-content-types")]
    type Ogg("audio/ogg"; Bytes)
    /// Represents the `video/mp4` content type.
    type Mp4("video/mp4"; Bytes)
    /// Represents the `audio/flac` content type.
    #[cfg(feature = "more-content-types")]
    type Flac("audio/flac"; Bytes)
    /// Represents the `audio/aac` content type.
    #[cfg(feature = "more-content-types")]
    type Aac("audio/aac"; Bytes)
    /// Represents the `audio/wav` content type.
    #[cfg(feature = "more-content-types")]
    type Opus("audio/opus"; Bytes)
    /// Represents the `audio/webm` content type.
    #[cfg(feature = "more-content-types")]
    type WebmVideo("video/webm"; Bytes)
    /// Represents the `audio/webm` content type.
    #[cfg(feature = "more-content-types")]
    type WebmAudio("audio/webm"; Bytes)
}
