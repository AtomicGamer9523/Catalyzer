use crate::*;

struct Config {
    init_func: Option<syn::Ident>,
}

impl syn::parse::Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut init_func = None;
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            if input.peek(syn::Token![=]) {
                input.parse::<syn::Token![=]>()?;
                let func: syn::Ident = input.parse()?;
                match ident.to_string().as_str() {
                    "init" => init_func = Some(func),
                    _ => return Err(syn::Error::new_spanned(ident, "Unknown configuration option")),
                }
            } else {
                match ident.to_string().as_str() {
                    "init" => init_func = Some(ident),
                    _ => return Err(syn::Error::new_spanned(ident, "Unknown configuration option")),
                }
            }
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Config {
            init_func,
        })
    }
}

pub(crate) fn main(cfg: T, input: T) -> T {
    let cfg = match syn::parse2::<Config>(cfg) {
        Err(e) => return e.to_compile_error(),
        Ok(c) => c,
    };

    let mut init_func = quote::quote!(None);
    if let Some(init) = cfg.init_func {
        init_func = quote::quote!(Some(#init));
    }

    let input = match syn::parse2::<syn::ItemFn>(input) {
        Err(e) => return e.to_compile_error(),
        Ok(f) => f,
    };

    let attrs = &input.attrs;
    let name = &input.sig.ident;
    let asyncness = &input.sig.asyncness;
    let fn_token = &input.sig.fn_token;
    let asyncness = asyncness.unwrap_or_else(|| syn::Token![async](proc_macro2::Span::call_site()));
    if name != "main" {
        return syn::Error::new_spanned(name, "only main function can be annotated with #[catalyzer::main]")
            .to_compile_error();
    }
    let body = &input.block;
    quote::quote!(
        #(#attrs)*
        #fn_token #name() {
            use ::catalyzer::internals::runtime::CatalyzerRuntime;
            #asyncness #fn_token #name() -> ::catalyzer::Result {
                #body.await?.await?;
                Ok(())
            }
            CatalyzerRuntime::init(#init_func).run(#name);
        }
    )
}
