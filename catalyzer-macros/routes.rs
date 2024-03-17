use syn::spanned::Spanned;
use super::*;

pub(crate) fn get(path: T, input: T) -> T {
    let mut func = match syn::parse2::<syn::ItemFn>(input) {
        Err(e) => return e.to_compile_error(),
        Ok(f) => f,
    };
    if func.sig.asyncness.is_none() {
        func.sig.asyncness = Some(syn::Token![async](func.span()));
    };
    let mut ret = func.sig.output.clone();
    match func.sig.output {
        syn::ReturnType::Default => {
            ret = syn::parse_quote! { -> impl ::catalyzer::res::IntoRawResponse }
        },
        _ => {}
    };
    let path = match syn::parse2::<syn::LitStr>(path) {
        Err(e) => return e.to_compile_error(),
        Ok(p) => p,
    };

    let asyncness = &func.sig.asyncness;
    let ident = &func.sig.ident;
    let block = &func.block;
    let vis = &func.vis;
    let attrs = &func.attrs;
    let generics = &func.sig.generics;
    let where_clause = &func.sig.generics.where_clause;
    let inputs = &func.sig.inputs;
    let output = &ret;

    let metadata_ident = syn::Ident::new(&format!("{}_metadata", ident), ident.span());
    let metadata_doc = syn::LitStr::new(&format!("Automatically generated metadata for the `{}` path handler.", ident), ident.span());
    let metadata = quote::quote! {
        #[doc = #metadata_doc]
        #[doc(hidden)]
        #[repr(transparent)]
        #[allow(non_camel_case_types)]
        struct #metadata_ident;
        impl ::catalyzer::__internals__::HandlerMetadata for #metadata_ident {
            const PATH: &'static str = #path;
            const METHOD: ::catalyzer::__internals__::Method = ::catalyzer::__internals__::Method::GET;
        }
    };

    quote::quote!(
        #(#attrs)*
        #vis #asyncness fn #ident #generics (#inputs) #output #where_clause {
            #block
        }
        #metadata
    )
}

pub(crate) fn post(cfg: T, input: T) -> T {
    input
}

pub(crate) fn put(cfg: T, input: T) -> T {
    input
}

pub(crate) fn delete(cfg: T, input: T) -> T {
    input
}

pub(crate) fn patch(cfg: T, input: T) -> T {
    input
}

pub(crate) fn head(cfg: T, input: T) -> T {
    input
}

pub(crate) fn options(cfg: T, input: T) -> T {
    input
}

pub(crate) fn trace(cfg: T, input: T) -> T {
    input
}
