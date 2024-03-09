extern crate proc_macro;

use proc_macro2::TokenStream as T;
use proc_macro::TokenStream;

mod main_func;

#[proc_macro_attribute]
pub fn main(cfg: TokenStream, input: TokenStream) -> TokenStream {
    main_func::main(cfg.into(), input.into()).into()
}

// #[proc_macro]
// #[cfg(feature = "html")]
// pub fn html(input: TokenStream) -> TokenStream {
//     html::html_macro(input.into()).into()
// }
