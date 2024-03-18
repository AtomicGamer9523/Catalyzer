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
