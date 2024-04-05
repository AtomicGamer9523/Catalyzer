use syn::spanned::Spanned;
use super::*;

const WATERMARK: &str = concat!("Automatically generated by Catalyzer v", env!("CARGO_PKG_VERSION"));

struct Watermark;
impl quote::ToTokens for Watermark {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(proc_macro2::Literal::string(WATERMARK).to_token_stream());
    }
}

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
        
            let metadata_ident = format!("{ident}_metadata");
            let metadata_ident = syn::Ident::new(&metadata_ident, ident.span());
            let metadata = quote::quote! {
                #[doc = #Watermark]
                #[doc(hidden)]
                #[repr(transparent)]
                #[allow(non_camel_case_types)]
                struct #metadata_ident;
                impl ::catalyzer::internals::HandlerMetadata for #metadata_ident {
                    const PATH: &'static str = #path;
                    const METHOD: ::catalyzer::internals::Method = ::catalyzer::internals::Method::$method;
                }
            };
        
            quote::quote!(
                #metadata
                #(#attrs)*
                #vis #asyncness fn #ident #generics (#inputs) #output #where_clause #block
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