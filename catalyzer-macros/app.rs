use syn::parse::*;
use crate::*;

type IdentList = syn::punctuated::Punctuated<syn::Ident, syn::Token![,]>;

pub(crate) fn app(input: T) -> T {
    let items = match IdentList::parse_terminated.parse2(input) {
        Err(e) => return e.to_compile_error(),
        Ok(i) => i,
    };
    let mut res = quote::quote!(::catalyzer::App::new());
    for item in items {
        let item_metadata = syn::Ident::new(&format!("{}_metadata", item), item.span());
        res = quote::quote!(#res.route::<_, #item_metadata, _>(#item)?);
    };
    res
}
