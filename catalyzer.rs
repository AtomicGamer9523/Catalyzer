#![crate_name = "catalyzer"]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/AtomicGamer9523/Catalyzer/main/.github/doc/logo.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/AtomicGamer9523/Catalyzer/main/.github/doc/logo.png")]
#![doc(html_root_url = "https://gh.matveit.dev/Catalyzer/")]
#![doc(html_no_source)]

//! Catalyzer is a web framework for Rust, made to be simple and easy to use.
//! 
//! # Example
//! 
//! ```rust
//! use catalyzer::*;
//! 
//! #[main]
//! fn main() {
//!     App![index]
//!         .bind("0.0.0.0:3000")?
//!         .launch()
//! }
//! 
//! #[get("/")]
//! fn index() {
//!     "Hello, world!"
//! }
//! ```

pub use ::base::*;
pub use ::macros::*;

/// Allows for simplistic creation of web applications.
#[macro_export(local_inner_macros)]
macro_rules! catalyze {
    [$($routes:ident)+] => (
        #[cfg(not(debug_assertions))]
        ::core::compile_error!(r#"You can only use the `catalyze!` macro in debug mode!

Hey there, it seems like you want to use the `catalyze!` macro in release mode.
Unfortunately, this is not possible, as it is intended to be used for development purposes only.

"#);
        #[$crate::main]
        #[cfg(debug_assertions)]
        fn main() { App![$($routes)+].__auto_configure()?.launch() }
        #[cfg(not(debug_assertions))]
        fn main() { loop { } }
    )
}
