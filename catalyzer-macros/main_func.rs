use crate::*;

pub(crate) fn main(cfg: T, input: T) -> T {
    if !cfg.is_empty() {
        return syn::Error::new_spanned(cfg, "unexpected configuration")
            .to_compile_error();
    }
    let input = match syn::parse2::<syn::ItemFn>(input) {
        Err(e) => return e.to_compile_error(),
        Ok(f) => f,
    };

    let name = &input.sig.ident;
    let asyncness = &input.sig.asyncness;
    let fn_token = &input.sig.fn_token;

    if asyncness.is_none() {
        return syn::Error::new_spanned(fn_token, "expected async function")
            .to_compile_error();
    }
    if name != "main" {
        return syn::Error::new_spanned(name, "expected `async fn main`")
            .to_compile_error();
    }

    let body = &input.block;

    quote::quote!(
        #fn_token #name() -> ::catalyzer::Result {
            let runtime = ::catalyzer::__internals__::runtime::Runtime::init()?;
            async fn #name() -> ::catalyzer::Result {
                #body.await?.await.map_err(From::from)
            }
            runtime.run_main(#name())
        }
    ).into()
}
