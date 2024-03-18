use syn::spanned::Spanned;
use super::*;

macro_rules! routes {
    ($($name:ident($method:ident))+)=>($(
        pub(crate) fn $name(path: T, input: T) -> T {
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
                    const METHOD: ::catalyzer::__internals__::Method = ::catalyzer::__internals__::Method::$method;
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
    )+)
}

routes! {
    get(GET)
    post(POST)
    put(PUT)
    delete(DELETE)
    patch(PATCH)
    head(HEAD)
    options(OPTIONS)
    trace(TRACE)
}
